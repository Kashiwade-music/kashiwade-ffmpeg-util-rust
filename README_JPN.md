# k-ffmpeg
<p align="center">
  <b>I can't remember how to use ffmpeg! Ugh!</b>
</p>

![Alt text](doc/head.png)

## About
ffmpegのシンプルな補助ツール。事前にyamlファイルにコマンドを書いておけば、`kffmpeg`とタイプするだけでインタラクティブにffmpegを実行できます。もうffmpegのコマンドをググる必要はありません。

## Installation
### Windows/Linux/Mac
- Releaseページから最新のプログラムをダウンロード。
- 適当な所に実行ファイルを置き、パスを通す。

## Basic Usage
1. ターミナルを開き、`kffmpeg`コマンドを実行します。初回実行時は初期設定ファイルを`~/.config/kffmpeg`に生成します。また、標準出力にもパスが書かれます。

    ```
    [  NG  ] Config file was not found. -> make at C:\Users\〇〇\.config\kffmpeg\config.yaml
    [  OK  ] Config loaded.
        13920aa1 -> Make video lighter by using h264_nvenc CQ 32
        c8ed2887 -> Concat videos by getting txt file
    [  OK  ] You did not specify --hash and --input_path. So, kffmpeg will run with user interaction.
    [  OK  ] ffmpeg command found
    ```

2. 設定ファイルをテキストエディタで開いてください。設定ファイルはYAMLで書かれており、各項目の説明は以下の通りです。

    ```yaml
    ffmpeg_path: /usr/bin/ffmpeg  # ffmpegコマンドが使用できない時に、直接ffmpeg実行ファイルのパスを設定できます。
    commands:
      - title: Make video lighter by using h264_nvenc CQ 32  # コマンドの短い説明です。
        options:
          - flag: -cq  # コマンドオプションのflagを設定します。
            value: 32  # コマンドオプションの値を設定します。
          - flag: -c:v
            value: h264_nvenc
        output_extension: .mp4  # 出力ファイルの拡張子を設定します。
        output_filename_suffix: _light  # 出力ファイルの接尾辞を設定します。例えば入力ファイル名がinput.mp4の時、出力ファイル名はinput_light.mp4になります。
        command:
          - "{{ffmpeg_path}}"  # 実際のコマンドを配列で入力します。{{ffmpeg_path}}は特殊なコマンドで、実行時に実行ファイル名に置換されます。必須です。
          - -i
          - "{{input_path}}"  # {{input_path}}は特殊なコマンドで、入力パスに置換されます。必須です。
          - "{{options}}"  # {{options}}は特殊なコマンドで、オプション系に置換されます。必須です。
          - "{{output_path}}"  # {{output_path}}は特殊なコマンドで、出力パスに置換されます。必須です。
        ...
    ```

3. 設定ファイルを保存し、ターミナルで`kffmpeg`を実行してください。後はプログラムからのメッセージに従えばffmpegを実行できます！

## Non-interactive Usage
`--hash`オプションと`--input-path`オプションを設定することで、応答を入力せずに実行できます。

1. ターミナルを開き、`kffmpeg`コマンドを実行します。`Config loaded.`の行の下に、コマンドタイトルとハッシュ値(8文字のランダムな文字)が表示されるはずです。

    ```
    [  OK  ] Config file found at C:\Users\ryo\.config\kffmpeg\config.yaml
    [  OK  ] Config loaded.
        13920aa1 -> Make video lighter by using h264_nvenc CQ 32
        c8ed2887 -> Concat videos by getting txt file
    [  OK  ] You did not specify --hash and --input_path. So, kffmpeg will run with user    interaction.
    [  OK  ] ffmpeg command found
    ```

2. Ctrl+C等を押し、k-ffmpegを終了します。その後、実行したいコマンドのハッシュ値を`--hash`オプションに、入力したいファイルのパスを`--input-path`に渡して、再度k-ffmpegを実行します。
   例えば、`Make video lighter by using h264_nvenc CQ 32`を`C:\movie.mp4`に適応させたいときは以下のようにターミナルに入力します。

    ```sh
    kffmpeg --hash 13920aa1 --input-path "C:\movie.mp4"
    ```