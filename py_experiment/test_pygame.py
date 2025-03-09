import pygame
import numpy as np

# Initialisation de Pygame Mixer
pygame.mixer.init(frequency=44100, size=-16, channels=1, buffer=1024)

# Paramètres du son
SAMPLE_RATE = 44100  # Hz
FREQUENCY = 440.0  # Hz (La4)
AMPLITUDE = 4096  # Amplitude du son (max 32767 pour 16-bit)
BUFFER_SIZE = 1024  # Taille du buffer audio


# Fonction pour générer une onde sinusoïdale
def generate_sine_wave(frequency, sample_rate, buffer_size):
    t = np.arange(buffer_size) / sample_rate
    wave = (AMPLITUDE * np.sin(2 * np.pi * frequency * t)).astype(np.int16)
    return np.column_stack([wave])  # Convertir en tableau compatible pygame


# Création d’un premier buffer audio
waveform = generate_sine_wave(FREQUENCY, SAMPLE_RATE, BUFFER_SIZE)

# Création du son Pygame avec les données initiales
sound = pygame.sndarray.make_sound(waveform)

# Lecture du son en boucle
sound.play(-1)

print("Son en temps réel... Appuyez sur Entrée pour changer la fréquence.")

try:
    while True:
        # Demander une nouvelle fréquence à l’utilisateur
        new_freq = input("Nouvelle fréquence (Hz) : ")
        try:
            FREQUENCY = float(new_freq)
            waveform = generate_sine_wave(FREQUENCY, SAMPLE_RATE, BUFFER_SIZE)
            sound = pygame.sndarray.make_sound(waveform)
            sound.play(-1)  # Redémarrer le son
        except ValueError:
            print("Entrez une valeur numérique valide.")
except KeyboardInterrupt:
    print("\nArrêt du son.")
    pygame.mixer.quit()
