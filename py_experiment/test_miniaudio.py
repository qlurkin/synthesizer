import miniaudio
import numpy as np

SAMPLE_RATE = 44100  # Hz
FREQUENCY = 440.0  # Hz (La4)


def sine_oscilator(frequency: float):
    frames = yield np.array([])
    phase = 0.0
    phase_increment = (2.0 * np.pi * frequency) / SAMPLE_RATE

    while True:
        samples = np.sin(phase + np.arange(frames) * phase_increment)
        phase = (phase + frames * phase_increment) % (2.0 * np.pi)
        frames = yield samples


def ads(attack: float, decay: float, sustain: float):
    frames = yield np.array([])
    elapsed_frames = 0


def no_sound():
    frames = yield np.array([])
    while True:
        frames = yield np.zeros(frames)


NO_SOUND = no_sound()
NO_SOUND.send(None)

tracks = [NO_SOUND] * 8
master_volume = 0.5


def noise_maker():
    frames = yield b""

    while True:
        samples = np.zeros(frames)
        for track in tracks:
            samples += track.send(frames)
        samples *= master_volume
        frames = yield samples.astype(np.float32).tobytes()


device = miniaudio.PlaybackDevice(
    output_format=miniaudio.SampleFormat.FLOAT32, sample_rate=SAMPLE_RATE, nchannels=1
)

noise = noise_maker()

noise.send(None)  # same as next(sine) to start generator

device.start(noise)

input("Appuyez sur Entrée pour arrêter...\n")

sine = sine_oscilator(FREQUENCY)
sine.send(None)

tracks[0] = sine

input("Appuyez sur Entrée pour arrêter...\n")

sine = sine_oscilator(FREQUENCY * 0.5)
sine.send(None)

tracks[1] = sine

input("Appuyez sur Entrée pour arrêter...\n")
device.stop()
