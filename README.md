<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br>fiberslice-5d
</h1>
<h3>◦ Unleash the 5D power with fiberslice</h3>
<h3>◦ Developed with the software and tools listed below.</h3>

<p align="center">
<img src="https://img.shields.io/badge/PowerShell-5391FE.svg?style&logo=PowerShell&logoColor=white" alt="PowerShell" />
<img src="https://img.shields.io/badge/Python-3776AB.svg?style&logo=Python&logoColor=white" alt="Python" />
<img src="https://img.shields.io/badge/Dlib-008000.svg?style&logo=Dlib&logoColor=white" alt="Dlib" />
<img src="https://img.shields.io/badge/Rust-000000.svg?style&logo=Rust&logoColor=white" alt="Rust" />
<img src="https://img.shields.io/badge/JSON-000000.svg?style&logo=JSON&logoColor=white" alt="JSON" />
<img src="https://img.shields.io/badge/Markdown-000000.svg?style&logo=Markdown&logoColor=white" alt="Markdown" />
</p>
<img src="https://img.shields.io/github/languages/top/GodGotzi/fiberslice-5d?style&color=5D6D7E" alt="GitHub top language" />
<img src="https://img.shields.io/github/languages/code-size/GodGotzi/fiberslice-5d?style&color=5D6D7E" alt="GitHub code size in bytes" />
<img src="https://img.shields.io/github/commit-activity/m/GodGotzi/fiberslice-5d?style&color=5D6D7E" alt="GitHub commit activity" />
<img src="https://img.shields.io/github/license/GodGotzi/fiberslice-5d?style&color=5D6D7E" alt="GitHub license" />
</div>

---

## 📒 Table of Contentss

- [📒 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [⚙️ Features](#-features)
- [📂 Project Structure](#project-structure)
- [🧩 Modules](#modules)
- [🚀 Getting Started](#-getting-started)
- [🗺 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [👏 Acknowledgments](#-acknowledgments)

---

## 📍 Overview

The project is a 3D application with a GUI interface for rendering and interacting with objects. It includes functionalities for handling window events, managing tasks, and providing a visualizer. The project also supports G-Code parsing, layer generation, and toolpath creation for 3D printing. Its value proposition lies in its ability to provide a comprehensive interface for 3D modeling, visualization, and printing, enhancing the user's workflow and productivity.

---

## 📂 Project Structure

---

## 🧩 Modules

# Seperate Systems

| File                                                                                                                   | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [create_protocol.py](https://github.com/GodGotzi/fiberslice-5d/blob/main/automated_prepare_gpt\create_protocol.py)     | This code is a command-line tool for creating commit protocols. It reads git commit logs, filters out relevant lines based on certain keywords, and generates protocol files for each commit. It uses OpenAI's GPT-3.5 Turbo model to describe the commit messages. The tool allows the user to specify a filename, API key, and an optional date filter for the commits. The generated protocol files are stored in separate folders for raw commits and processed commits. |
| [delete_all_diffs.ps1](https://github.com/GodGotzi/fiberslice-5d/blob/main/automated_prepare_gpt\delete_all_diffs.ps1) | The code retrieves a list of files from a "diffs" directory and then proceeds to delete each file forcefully.                                                                                                                                                                                                                                                                                                                                                                |

# Main

| File                                                                                                                   | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------                                                                                                                                                                                                   |
| [application.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\application.rs)                               | The code defines an Application struct with various methods for handling window events, tasks, and a visualizer. It also includes a TaskHandler struct for managing tasks, and an ApplicationContext struct for managing the application's context, theme, mode, and boundaries. The code includes a function for rendering the user interface and a few helper structs and enums.                                                                                                         |
| [config.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\config.rs)                                         | This code defines constants and functions for GUI styling and sizes used in the project. It includes a default window size, color scheme selection, and measurements for menu bars, mode bars, task bars, and toolbars. It also has a module for potential addons and a nested module for default settings bar width.                                                                                                                                                                      |
| [error.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\error.rs)                                           | The code defines an enum called `Error` with different variants representing various error types. It includes functions to format and display error messages. These error types cover common errors related to generic, missing fields, initial build, GCode parsing, unknown instruction types, setup errors, GCode state parsing, and IO errors.                                                                                                                                         |
| [gui.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui.rs)                                               | The code is a collection of GUI components like menu bars, task bars, mode bars, settings bars, and toolbars. It also includes a screen struct that represents the entire user interface and has methods to show these components. The goal is to provide a comprehensive summary of the code's core functionalities in a concise manner.                                                                                                                                                  |
| [main.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\main.rs)                                             | This code sets up a 3D application with a GUI interface for rendering and interacting with objects. It handles input events, updates the scene, performs rendering, and manages the main event loop. It also includes a test function to add objects to the scene.                                                                                                                                                                                                                         |
| [prelude.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\prelude.rs)                                       | This code defines two structs, `AsyncPacket` and `AsyncWrapper`, which handle asynchronous data. The `AsyncPacket` struct contains two optional elements, `sync_element` and `async_element`, which can hold an item of type `Item`. The `AsyncWrapper` struct manages a vector of `AsyncPacket` instances and provides methods to manipulate and access the data. One notable feature is the ability to find and register items within the `AsyncWrapper` struct.                         |
| [setup.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\setup.rs)                                           | This code defines a setup context for different 3D printers. It stores information like box offset, printing box dimensions, and printer GLB file path. The code also provides a conversion function to load the setup configuration from a YAML file based on the selected printer setup.                                                                                                                                                                                                 |
| [window.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\window.rs)                                         | This code builds a window using the winit library. It handles different platforms, sets window properties, and returns a Result containing the built window or an error message.                                                                                                                                                                                                                                                                                                           |
| [icon.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\icon.rs)                                         | The code defines a struct IconTable that holds different icon images for various orientations. It provides a method to retrieve the icon based on the given orientation and also handles loading the icons from disk using the image crate.                                                                                                                                                                                                                                                |
| [menubar.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\menubar.rs)                                   | The code defines a menubar GUI component with buttons for File, Edit, View, Settings, and Help. It uses the egui library to create the UI and handles user interactions through callbacks. The menubar is displayed at the top of the application window.                                                                                                                                                                                                                                  |
| [modebar.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\modebar.rs)                                   | The code defines the Modebar component, which displays a mode selection bar in a user interface. It uses egui_extras, egui_grid, and three_d libraries for UI rendering and interaction. The component's show function builds the mode selection bar using an egui::Ui object and custom layout. It allows the user to select different modes and updates the application's context accordingly.                                                                                           |
| [settingsbar.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\settingsbar.rs)                           | The code provides a settings panel with tabs for different settings categories (slice, filament, and printer). It uses the egui library to create a user interface and allows users to switch between different settings sections based on their selection. The Settingsbar struct is responsible for managing the open_panel state and displaying the appropriate settings based on the current selection.                                                                                |
| [taskbar.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\taskbar.rs)                                   | This code defines a Taskbar component that can be shown in an egui GUI. It displays a bar at the bottom with FPS counter and a theme toggle button. The Taskbar responds to user input and updates the GUI accordingly.                                                                                                                                                                                                                                                                    |
| [toolbar.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\toolbar.rs)                                   | The code defines a `Toolbar` struct and implements the `Component` trait for it. The `show` function uses egui to display a side panel called "toolbar" with a defined width. It also registers an event for changing the toolbar width. The resulting boundary is stored in the application context.                                                                                                                                                                                      |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\mod.rs)                                | The code allows for the inclusion and management of addons in a modular fashion, enhancing the overall functionality of the system.                                                                                                                                                                                                                                                                                                                                                        |
| [force_analytics.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\addons\force_analytics.rs) | This code defines a function called `show` that displays a complex graphical user interface using the egui library. The interface consists of multiple nested panels arranged horizontally and vertically, with some panels having specific dimensions. The code also applies a shaded color based on the dark mode setting. The function is executed by a callback function passed as an argument. Overall, it creates a visually appealing and interactive interface for an application. |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\addons\mod.rs)                         | The code defines functions and modules related to creating graphical user interfaces. It includes the creation of addon strips, displaying orientation buttons, and handling different modes in the GUI.                                                                                                                                                                                                                                                                                   |
| [monitor.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\addons\monitor.rs)                 | The code defines a function that generates a GUI using egui library. It builds a complex layout using various sizes and orientations. The resulting GUI is a combination of different components and styles defined in the application context. The GUI also includes interaction with the user, such as handling user input and displaying visual elements.                                                                                                                               |
| [prepare.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\addons\prepare.rs)                 | The code displays a GUI component using the egui library. It creates a complex layout with various sizes and strips, including nested strips. The GUI component is created using a builder pattern and registered with a bounding box.                                                                                                                                                                                                                                                     |
| [preview.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\gui\components\addons\preview.rs)                 | The code shows a GUI panel using the egui framework. It creates a layout with various sizes and elements, including a colored rectangle and a nested structure. The boundary object is registered for GUI interaction.                                                                                                                                                                                                                                                                     |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\math\mod.rs)                                          | This code defines a VirtualPlane struct that represents a mathematical plane in a 3D space. It has a position and a normal vector. It also provides methods to access the position and normal vectors.                                                                                                                                                                                                                                                                                     |
| [layer.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\layer.rs)                                     | The code provides functionalities for creating and manipulating 3D mesh objects. It includes features for adding triangles, drawing paths and rectangles, and constructing layer models. Additionally, it defines structs and methods for managing mesh elements and coordinates.                                                                                                                                                                                                          |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\mod.rs)                                         | The code includes modules for G-code parsing (gcode) and layer generation (layer). It efficiently handles the core functionalities with precision and conciseness.                                                                                                                                                                                                                                                                                                                         |
| [instruction.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\instruction.rs)                   | This code defines several structs and enums related to CNC machine instructions. It includes functionality for parsing and representing different instruction types, storing instructions along with their child instructions and movements, and generating G-code from the instructions.                                                                                                                                                                                                  |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\mod.rs)                                   | The code provides a SourceBuilder struct for constructing G-code source strings. It supports adding movements and instructions, and can output the final source string. The GCode struct is used to represent a collection of instruction modules.                                                                                                                                                                                                                                         |
| [movement.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\movement.rs)                         | This code defines a'Movements' struct with fields X, Y, Z, E, and F. It allows setting and adding movements, converting the movements to a vector, and generating G-Code strings.                                                                                                                                                                                                                                                                                                          |
| [parser.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\parser.rs)                             | The code provides functionality to parse and convert G-code instructions into a structured representation. It handles comments, instructions, and parameters. The resulting structured representation is wrapped in a GCode struct.                                                                                                                                                                                                                                                        |
| [state.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\state.rs)                               | The code defines an enum called StateField, representing different fields of a state. It also provides a conversion implementation from a string to the StateField enum. The State struct holds optional values for layer, print type, and mesh. It has a parse method that takes a string input, converts it into a StateField enum, and assigns the corresponding value to the State struct's fields.                                                                                    |
| [toolpath.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\model\gcode\toolpath.rs)                         | The code is used to generate tool paths for 3D printing based on G-code instructions. It processes the instructions, calculates the path lines, and organizes them into layers. The code also includes functionality for rendering and creating meshes for each layer.                                                                                                                                                                                                                     |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\slicer\mod.rs)                                        | The code consists of a module called "print_type" that likely includes functions or structures related to printing or handling different types of data in a program.                                                                                                                                                                                                                                                                                                                       |
| [print_type.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\slicer\print_type.rs)                          | The code defines an enum called "PrintType" with different print types. It provides a function to get the color associated with each print type using the "Srgba" struct from the "three_d_asset" crate. Each print type has a unique hard-coded color.                                                                                                                                                                                                                                    |
| [format.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\utils\format.rs)                                   | This code defines a `PrettyFormat` trait and several implementations for formatting different types of vectors and numeric values. It provides a method `pretty_format` that converts the values to a string representation.                                                                                                                                                                                                                                                               |
| [frame.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\utils\frame.rs)                                     | The code defines a FrameHandle trait with a frame function. This function takes in a FrameInput and a shared reference to an Application. Its purpose is to handle frame logic.                                                                                                                                                                                                                                                                                                            |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\utils\mod.rs)                                         | This code defines modules for formatting, frames, and tasks. It also provides a debug wrapper and implementation for flipping the y and z coordinates of a Vector3 data structure.                                                                                                                                                                                                                                                                                                         |
| [task.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\utils\task.rs)                                       | This code defines a `TaskWithResult` struct that allows running a task asynchronously and retrieving its result. It also provides a way to kill the task if needed. The `TaskWithResult` struct uses `tokio` for asynchronous execution and employs the `oneshot` channel to receive the task's result.                                                                                                                                                                                    |
| [buffer.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\buffer.rs)                                    | This code defines a generic ObjectBuffer struct that manages a collection of objects and models. It provides methods to add, remove, hide, show, and retrieve objects and models. It also supports rendering and picking functionality. It uses hideable objects to control object visibility.                                                                                                                                                                                             |
| [camera.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\camera.rs)                                    | The code defines a trait and implements it for the `Camera` struct. It also includes a `CameraBuilder` struct to construct a camera with various configuration options. By calling the `handle_orientation()` method on a camera object, its view is set based on the orientation provided.                                                                                                                                                                                                |
| [environment.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\environment.rs)                          | This code defines the `Environment` struct, which holds a camera, camera control, and a list of lights. It provides functions to access and modify the camera, handle camera events, and update the viewport based on the frame input.                                                                                                                                                                                                                                                     |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\mod.rs)                                          | This code provides functionalities for handling buffers, cameras, environments, and visualizations in a three-dimensional space. It also includes implementations for the Contains trait, an enum for orientations, and an enum for different modes of operation.                                                                                                                                                                                                                          |
| [force.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\visualization\force.rs)                        | The `ForceVisualizer` struct has a `result` field, which is an optional shared mutex that wraps a task result. It provides a constructor method to create a new instance of the `ForceVisualizer` struct.                                                                                                                                                                                                                                                                                  |
| [mod.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\visualization\mod.rs)                            | This code defines a visualizer trait for rendering objects in an application. It includes modules for force visualization and model handling. The VisualizerContext struct manages different types of visualizers, specifically for GCode and force visualization. The code provides methods for accessing and manipulating these visualizers.                                                                                                                                             |
| [model.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/src\view\visualization\model.rs)                        | This code implements a GCode visualizer with the ability to set and visualize GCode instructions, generating 3D layers based on the instructions. It also includes functions for collecting and building test mesh objects.                                                                                                                                                                                                                                                                |
| [lib.rs](https://github.com/GodGotzi/fiberslice-5d/blob/main/traits\src\lib.rs)                                        | This code defines a trait called `TypeEq` which allows for type equality checks. It has one function, `type_eq`, that compares the type of an instance implementing the trait with another given type and returns true if they are equal, and false otherwise.                                                                                                                                                                                                                             |

---

## 🚀 Getting Started

### ✔️ Prerequisites

Before you begin, ensure that you have the following prerequisites installed:

> - `ℹ️ Requirement 1`
> - `ℹ️ Requirement 2`
> - `ℹ️ ...`

### 📦 Installation

1. Clone the fiberslice-5d repository:

```sh
git clone https://github.com/GodGotzi/fiberslice-5d
```

2. Change to the project directory:

```sh
cd fiberslice-5d
```

3. Install the dependencies:

```sh
cargo build
```

### 🎮 Using fiberslice-5d

```sh
cargo run
```

### 🧪 Running Tests

```sh
cargo test
```

---

## 🗺 Roadmap

> - [x] `ℹ️  Task 1: Implement X`
> - [ ] `ℹ️  Task 2: Refactor Y`
> - [ ] `ℹ️ ...`

---

## 👏 Acknowledgments

> - `ℹ️  List any resources, contributors, inspiration, etc.`

---
