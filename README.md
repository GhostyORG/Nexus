<h1 align="center">
  Nexus
</h1>

<p align="center">
Nexus, a cross-platform blazingly fast code/text editor with Syntax Highlighting
</p>

## Installation

Installing Nexus is very straight forward.<br>
Currently, Mac Support is pending due to the lack of testers.

Head over to the **[releases](https://github.com/GhostyORG/Nexus/releases)** and grab the `.msi` if you are on windows, `tar.zst` & `.deb` for Linux

## Development

If you'd like to compile Nexus from source, please read the following: <br>

**REQUIREMENTS**<br> 

- All prerequisites in **[here](https://tauri.app/v1/guides/getting-started/prerequisites/)**
- yarn because npm bad (`npm i -g yarn`)

Once met requirements, do the following:
```shell
$ git clone https://github.com/GhostyORG/Nexus.git && cd Nexus
$ yarn
```
To run Nexus afterwards, `yarn tauri:serve`.
