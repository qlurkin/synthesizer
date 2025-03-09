import miniaudio
import numpy as np

# Paramètres du son
SAMPLE_RATE = 44100  # Hz
FREQUENCY = 440.0  # Hz (La4)
AMPLITUDE = 0.5  # Volume (entre 0 et 1)
BUFFER_SIZE = 512  # Nombre d'échantillons par callback

class SineWaveGenerator:
    def __init__(self, sample_rate, frequency, amplitude):
        self.sample_rate = sample_rate
        self.frequency = frequency
        self.amplitude = amplitude
        self.phase = 0.0
        self.phase_increment = (2.0 * np.pi * self.frequency) / self.sample_rate

    def callback(self, frame_count: int, time_info, status):
        """ Génère une onde sinusoïdale en temps réel. """
        output_buffer = np.zeros(frame_count, dtype=np.float32)
        
        for i in range(frame_count):
            output_buffer[i] = self.amplitude * np.sin(self.phase)
            self.phase += self.phase_increment
            if self.phase >= 2.0 * np.pi:
                self.phase -= 2.0 * np.pi  # Évite la dérive de phase

        return output_buffer.tobytes()

# Création du générateur de son
sine_wave = SineWaveGenerator(SAMPLE_RATE, FREQUENCY, AMPLITUDE)

# Initialisation de la sortie audio avec un callback
with miniaudio.PlaybackDevice(output_format=miniaudio.SampleFormat.FLOAT32, sample_rate=SAMPLE_RATE, nchannels=1) as device:
    device.start(sine_wave.callback)

    print("Lecture en temps réel... Appuyez sur Ctrl+C pour arrêter.")
    try:
        while True:
            pass
    except KeyboardInterrupt:
        print("\nArrêt du son.")
