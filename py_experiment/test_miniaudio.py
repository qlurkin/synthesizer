import miniaudio
import numpy as np

# Paramètres audio
SAMPLE_RATE = 44100  # Hz
FREQUENCY = 440.0  # Hz (La4)
AMPLITUDE = 0.5  # Volume (entre 0 et 1)
BUFFER_SIZE = 512  # Nombre d'échantillons par bloc

def sine_wave_generator():
    """ Générateur d'onde sinusoïdale en temps réel pour miniaudio """
    phase = 0.0
    phase_increment = (2.0 * np.pi * FREQUENCY) / SAMPLE_RATE

    while True:
        # Génération du buffer de samples
        samples = (AMPLITUDE * np.sin(phase + np.arange(BUFFER_SIZE) * phase_increment)).astype(np.float32)
        phase = (phase + BUFFER_SIZE * phase_increment) % (2.0 * np.pi)
        
        # miniaudio attend un buffer sous forme de bytes
        yield samples.tobytes()

# Création du périphérique audio
device = miniaudio.PlaybackDevice(output_format=miniaudio.SampleFormat.FLOAT32, sample_rate=SAMPLE_RATE, nchannels=1)

sine = sine_wave_generator()

next(sine)

# Démarrer la lecture en utilisant le générateur
device.start(sine)

# Maintenir l’exécution
input("Appuyez sur Entrée pour arrêter...\n")
device.stop()
