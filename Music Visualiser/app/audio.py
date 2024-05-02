from pydub import AudioSegment
import librosa
import json


class AudioConverter:
    def __init__(self, audio_file: str):
        self.audio = audio_file
        self.base_path = "audio/"

    def get_extension(self) -> str:
        """
        Get extension of an audio.
        :return extension of gave audio:
        """
        return self.audio.split(".")[-1]

    def get_audio_name(self) -> str:
        """
        Split audio by path and extension
        :return audio name:
        """
        return self.audio.split("/")[-1].split(".")[0]

    def convert_to_wav(self) -> str:
        """
        Convert mp3 [etc] to wav.
        :return audio path:
        """
        if self.get_extension() == "wav":
            return self.audio
        elif self.get_extension() == "mp3":
            print(f"{self.base_path}{self.audio}")
            audio = AudioSegment.from_mp3(f"{self.base_path}{self.audio}")
            audio.export(f"{self.base_path}{self.get_audio_name()}.wav", format="wav")


class AudioParser:
    def __init__(self):
        self.base_path: str = "audio/"
        self.json_names_file: str = "settings/malters_metronome.json"

    def get_tempo_name(self, tempo: int) -> str:
        """
        Get tempo name by Malters metronome
        :param tempo:
        :return tempo name | str:
        """
        with open(self.json_names_file) as file:
            data: dict = json.load(file)
            for tempo_name, tempo_borders in data.items():
                if tempo_borders[0] < tempo <= tempo_borders[1]:
                    return tempo_name

    def parse_audio(self, audio: str) -> None:
        """
        Parse audio to get tempo and tempo name
        :param audio:
        :return None:
        """
        converter = AudioConverter(audio)
        converter.convert_to_wav()
        y, sr = librosa.load(f"{self.base_path}{audio}")
        tempo: int = int(librosa.beat.tempo(y=y, sr=sr)[0])
        print(f"Tempo: {tempo}, tempo name: {self.get_tempo_name(tempo)}")
