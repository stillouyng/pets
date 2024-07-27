import configparser
import os
import time
import wave
from pathlib import Path

import keyboard
import pyaudio
from screen_recorder_sdk import screen_recorder


def is_folder_exists() -> bool:
    if not os.path.exists('Audio'):
        os.makedirs('Audio')

    if not os.path.exists("Video"):
        os.makedirs("Video")

    if not Path("config.ini").exists():
        file = open("config.ini", "x")
        file.close()
    return True


def show_audio_devices():
    p = pyaudio.PyAudio()
    info = p.get_host_api_info_by_index(0)
    numdevices = info.get('deviceCount')
    for i in range(0, numdevices):
        if (p.get_device_info_by_host_api_device_index(0, i).get('maxInputChannels')) > 0:
            print("Input Device id ", i, " - ", p.get_device_info_by_host_api_device_index(0, i).get('name'))


def read_config() -> str | bool:
    config = configparser.ConfigParser()
    config.read('config.ini')
    try:
        return config['DEFAULT']['AudioDevice']
    except Exception:
        print("Выбери id микрофона")
        print("И введи его в config.ini :)\n")
        show_audio_devices()
        input("")
        return False


def get_audio_device() -> str | bool:
    file = Path("config.ini")
    if file.exists():
        return read_config()
    else:
        return False

def main():
    audio_device = get_audio_device()
    if audio_device:
        now = time.strftime("%Y-%m-%d-%H-%M-%S")
        screen_recorder.enable_dev_log()

        params = screen_recorder.RecorderParams()

        screen_recorder.init_resources(params)

        print('Video Started')
        filename = "Audio/{}.mp3".format(now)
        chunk = 1024
        FORMAT = pyaudio.paInt16
        channels = 1
        sample_rate = 44100

        p = pyaudio.PyAudio()
        stream = p.open(
            input_device_index=int(audio_device),
            format=FORMAT,
            channels=channels,
            rate=sample_rate,
            input=True,
            output=True,
            frames_per_buffer=chunk
        )
        frames = []

        print("Audio Recording...")
        screen_recorder.start_video_recording('Video/{}.mp4'.format(now), 60, 9000000, True)

        while True:
            data = stream.read(chunk)
            frames.append(data)
            if keyboard.is_pressed('F9'):
                break

        while True:
            try:
                if keyboard.is_pressed('F9'):
                    p.terminate()
                    wf = wave.open(filename, "wb")
                    wf.setnchannels(channels)
                    wf.setsampwidth(p.get_sample_size(FORMAT))
                    wf.setframerate(sample_rate)
                    wf.writeframes(b"".join(frames))
                    wf.close()
                    screen_recorder.stop_video_recording()
                    stream.stop_stream()
                    stream.close()
                    print("Audio finished recording.")
                    print('Video Stopped')
                    break
            except KeyboardInterrupt:
                break


if __name__ == "__main__":
    is_folder_exists()
    main()
