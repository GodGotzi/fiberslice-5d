import sys
import os
from enum import Enum
from git import Repo

DEBUG = False

def debug_print(str):
   if DEBUG:
      print(str)

class TokenPosition(Enum):
   STARTS = 0
   ENDS = 1
   ANYWHERE = 2

class Token:
   def __init__(self, token: str, flag=True, position=TokenPosition.ANYWHERE):
      self.token = token
      self.flag = flag
      self.position = position
   
class TokenArmy:
   def __init__(self, tokens: list):
      self.tokens = tokens

def check_token(line, token: Token):
   if token.position == TokenPosition.STARTS:
      return line.startswith(token.token)
   elif token.position == TokenPosition.ENDS:
      return line.endswith(token.token)
   else:
      return token.token in line

def contains_all(line: list, tokens: list):
   for token in tokens:

      if token.flag:
         if not check_token(line, token):
            return False
      else:
         if check_token(line, token):
            return False
         
   return True

def contains_anything(line: list, token_filters: list):
   for token_army in token_filters:
      if contains_all(line, token_army.tokens):
         return True
   return False

def check_reset(line: list, tokens: list):
   return contains_all(line, tokens)

def check_hit(line: list, token_filters: list):
   return contains_anything(line, token_filters)

def record_new_lines(lines, token_filters, reset_token):
   commits = [[]]
   commit_index = 0
   record = True

   for line in lines:
      if check_token(line, Token('commit ', position=TokenPosition.STARTS)):
         commit_index += 1
         commits.append([])
         debug_print("New commit: " + str(commit_index))

      if check_reset(line, reset_token.tokens):
         record = True

      if check_hit(line, token_filters):
         record = False
      
      if record:
         commits[commit_index].append(line)
         commits[commit_index].append('\n')

   return commits

token_filters = [
   TokenArmy([Token('[package]')]), 
   TokenArmy([Token('diff ', position=TokenPosition.STARTS), Token('.rs', flag=False)]),
]

if len(sys.argv) < 2:
   print("Usage: python create_protocol.py [filename] [since?optional]")
   exit()

repo: Repo = Repo("../")

if len(sys.argv) > 2:
   since = '--since=' + sys.argv[2]
else:
   since = '--since=2022-01-01'

print("Since: " + since)
# Retrieve the git log
lines = repo.git.log("-p", since).splitlines()

commits = record_new_lines(
   lines, 
   token_filters, 
   TokenArmy([Token('diff ', position=TokenPosition.STARTS)])
)

if not os.path.exists(sys.argv[1]):
   os.makedirs(sys.argv[1])

for i in range(len(commits)):
   file = open(sys.argv[1] + '/' + str(i) + '.txt', "w", encoding='utf-16-le')
   file.writelines(commits[i])
   file.close()
