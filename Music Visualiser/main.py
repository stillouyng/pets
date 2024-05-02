import configparser
from app.audio import AudioParser
import os


class App:

    def __init__(self):
        self.audio = AudioParser()
        self.base_path: str = "audio/"
        self.config = None

    def get_all_audios(self):
        audios = [audio for audio in os.listdir(self.base_path) if os.path.isfile(os.path.join(self.base_path, audio))]
        return audios

    def run(self):
        audios: list[str] = self.get_all_audios()
        self.audio.parse_audio(audios[0])


if __name__ == "__main__":
    app = App()
    app.run()
