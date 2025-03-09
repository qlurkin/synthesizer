import soundcard as sc
import numpy as np

# Paramètres audio
SAMPLE_RATE = 44100  # Hz
FREQUENCY = 440.0  # Hz (La4)
AMPLITUDE = 0.5  # Volume (entre 0 et 1)
BUFFER_SIZE = 512  # Nombre d'échantillons par buffer

# Sélection du périphérique de sortie par défaut
speaker = sc.default_speaker()

# Génération de l'onde sinusoïdale en temps réel
phase = 0.0
phase_increment = (2.0 * np.pi * FREQUENCY) / SAMPLE_RATE


def generate_sine_wave(frames: int):
    """Génère un buffer d'onde sinusoïdale"""
    global phase
    t = np.arange(frames) / SAMPLE_RATE
    wave = AMPLITUDE * np.sin(2 * np.pi * FREQUENCY * t + phase)
    phase += frames * phase_increment
    return wave.astype(np.float32)  # soundcard attend du float32


# Lecture du son en streaming
print("Lecture en cours... Appuyez sur Ctrl+C pour arrêter.")
try:
    with speaker.player(samplerate=SAMPLE_RATE) as player:
        while True:
            samples = generate_sine_wave(BUFFER_SIZE)
            player.play(samples)
except KeyboardInterrupt:
    print("\nArrêt du son.")
