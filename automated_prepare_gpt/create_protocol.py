import sys
import os
from enum import Enum
from git import Repo
import openai

if len(sys.argv) < 3:
   print("Usage: python create_protocol.py [filename] [api_key] [since?optional]")
   exit()

if not os.path.exists(sys.argv[1]):
   os.makedirs(sys.argv[1])

openai.api_key = sys.argv[2]

DEBUG = False

def split(list_a, chunk_size):

  for i in range(0, len(list_a), chunk_size):
    yield list_a[i:i + chunk_size]

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

class Commit:
   def __init__(self, commit: list[str]):
      self.commits: list[list[str]] = list(split(commit, 900))

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

def record_new_lines(lines: list[str], token_filters, reset_token):
   commits: list[Commit] = []
   commit_index = -1
   record = True

   for line in lines:
      if check_token(line, Token('commit ', position=TokenPosition.STARTS)):
         commits.append(Commit([]))
         commit_index += 1
         debug_print("New commit: " + str(commit_index))

      if check_reset(line, reset_token.tokens):
         record = True

      if check_hit(line, token_filters):
         record = False
      
      if record and commit_index >= 0:
         commits[commit_index].commits.append(line)
         commits[commit_index].commits.append('\n')

   return commits

def describe_commit(commits: list[Commit]):
   desc_results = []
   messages = []
   for commit in commits:
      result = ''

      for child_commit in commit.commits:
         messages.append({"role": "user", "content": "Please create a Commit Protocol for the following commit. \n\n" + (''.join(child_commit)) + "\n\n"})

         response = openai.ChatCompletion.create(
            model="gpt-3.5-turbo",
            messages=messages
         )

         for choice in response.choices:
            result += choice.message.content
      
      desc_results.append(result)

   return desc_results

def write_commit_files(commits: list[list[str]], child_folder: str):
   if not os.path.exists(sys.argv[1] + '/'+ child_folder):
      os.makedirs(sys.argv[1] + '/'+ child_folder)

   for i in range(len(commits)):
      file = open(sys.argv[1] + '/'+ child_folder + '/' + str(i) + '.txt', "w+", encoding='utf-8')
      file.write(commits[i])
      file.close()
 

token_filters = [
   TokenArmy([Token('[package]')]), 
   TokenArmy([Token('diff ', position=TokenPosition.STARTS), Token('.rs', flag=False)]),
]

repo: Repo = Repo("../")

if len(sys.argv) > 3:
   since = '--since=' + sys.argv[3]
else:
   since = '--since=2022-01-01'

print("Since: " + since)

lines = repo.git.log("-p", since).splitlines()

commits: Commit = record_new_lines(
   lines, 
   token_filters, 
   TokenArmy([Token('diff ', position=TokenPosition.STARTS)])
)

results = []

#results = describe_commit(commits)

print(results)

write_commit_files(commits, 'raw_commits')
write_commit_files(results, 'protocols')
