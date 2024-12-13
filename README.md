<p align="center">
  <img src=".github/imgs/ffust-no-bg.png" alt="ffust logo" width="200" height="200">
</p>

# :crab: ffust

![GitHub License](https://img.shields.io/github/license/antonioberna/ffust)
![GitHub Created At](https://img.shields.io/github/created-at/antonioberna/ffust)

## Introduction

This little project was born from the idea of sharing (without quality loss) video and/or audio files recorded via `OBS Studio` and/or `Audacity`.
In fact, these software are very powerful but at the same time they create really large files in order to be shared.
In addition the `ffmpeg` software has options that are not very easy to remember.

So the purpose of `ffust` is to implement the operations of audio capture from video, lossless video compression and conversion from one video and/or audio format to another using English instead of flags that are incomprehensible to newbies.

## Requirements

First of all, you need to have `ffmpeg` installed on your system. On `Arch Linux` you can install it with the following command:

```
sudo pacman -S ffmpeg
```

> [!WARNING]
> If you are using another distribution, you can install `ffmpeg` with the package manager of your distribution.

## Download & Installation

You can clone the repository with the following command:

```
git clone https://github.com/AntonioBerna/ffust.git
```

Now, after navigate to the project folder, you can install the program in your system with the following command:

```
cargo install --path .
```

> [!NOTE]
> I recommend you to install the program in the system instead of using `cargo`. However, if you want to use `cargo`, you can run the program with the following command:
>    ```
>    cargo run -- <operation> <input-file> <output-file>
>    ```

## Uninstall

If you want to uninstall the program, you can do it with the following command:

```
cargo uninstall ffust
```

> [!NOTE]
> If you use `cargo` to run the program, you can uninstall it with the following command:
>    ```
>    cargo clean
>    ```

## Supported Operations

| Operation   | Description                                                                             |
| :---:       | :---:                                                                                   |
| `get-audio` | Extracts audio from a video file and saves it as an audio file.                         |
| `compress`  | Compresses a video using the `libx265` codec with a specified quality and preset speed. |
| `convert`   | Converts a video file to another format.                                                |

## Mini docs

After the installation, you can run the program with the following command:

```
ffust --help
```

You will see the following output:

```
🦀 ffust is a simple FFmpeg wrapper.

Usage: ffust <operation> <input-file> <output-file>

Arguments:
  <operation>    The operation to perform. [possible values: get-audio, compress, convert]
  <input-file>   Path to the input file.
  <output-file>  Path to the output file.

Options:
  -h, --help  Print help
```

For example, if you want to extract the audio from a video file, you can run the following command:

```
ffust get-audio video.mp4 audio.mp3
```

If you want to compress a video file, you can run the following command:

```
ffust compress video.mp4 video-compressed.mp4
```

If you want to convert a video file to another format, you can run the following command:

```
ffust convert video.mp4 video.avi
```
