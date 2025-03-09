import sounddevice as sd
import numpy as np

SAMPLE_RATE = 44100
FREQUENCY = 220.0
AMPLITUDE = 0.9

phase = 0.0
phase_increment = (2.0 * np.pi * FREQUENCY) / SAMPLE_RATE


def callback(outdata, frames, time, status):
    global phase
    t = np.arange(frames) / SAMPLE_RATE
    outdata[:, 0] = AMPLITUDE * np.sin(2 * np.pi * FREQUENCY * t + phase)
    phase += frames * phase_increment


stream = sd.OutputStream(samplerate=SAMPLE_RATE, channels=1, callback=callback)
stream.start()

input("Appuyez sur Entrée pour arrêter...\n")
stream.stop()
stream.close()
