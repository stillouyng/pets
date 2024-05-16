from PyQt5 import QtCore, QtGui, QtWidgets
from app.audio import AudioParser
from views import main_view
import configparser
import sys
import os


class App:

    def __init__(self):
        self.audio = AudioParser()
        self.base_path: str = "audio/"
        self.config = None
        self.create_main()

    def create_main(self):
        app = QtWidgets.QApplication(sys.argv)
        Main = QtWidgets.QMainWindow()
        ui = main_view.Ui_Main()
        ui.setupUi(Main)
        Main.show()
        sys.exit(app.exec_())

    def get_all_audios(self):
        audios = [audio for audio in os.listdir(self.base_path) if os.path.isfile(os.path.join(self.base_path, audio))]
        return audios

    def run(self):
        audios: list[str] = self.get_all_audios()
        self.audio.parse_audio(audios[0])


if __name__ == "__main__":
    app = App()
    app.run()
