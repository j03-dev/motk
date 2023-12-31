import motk
import time

start = time.time()
motk = motk.MotKFinder("DEM.json", "french_stopwords")
print(f"temps pour le chargement des fichers {time.time() - start}")



start = time.time()

r = motk.find_keywords("""La reconnaissance d'image est un domaine en constante évolution de l'intelligence artificielle qui suscite un intérêt croissant. Elle permet aux ordinateurs de comprendre, d'analyser et d'interpréter des images visuelles, simulant ainsi la capacité de vision humaine. Dans cette rédaction, nous explorerons les principes fondamentaux de la reconnaissance d'image et nous examinerons l'évolution de l'approche, en passant de l'algorithmie classique au Machine Learning.
La reconnaissance d'image repose sur la représentation et le traitement des images par les ordinateurs. Nous aborderons la conversion des images en niveaux de gris ou en couleurs, la résolution des images, ainsi que la représentation des pixels. De plus, nous explorerons les opérations de base telles que la convolution, la binarisation et les opérations logiques, qui sont couramment utilisées dans les algorithmes classiques de traitement d'image.
L'algorithme de pourcentage de pixel est l'un des exemples d'algorithmes classiques utilisés pour évaluer la composition d'une image en termes de pixels blancs. Nous examinerons son fonctionnement, ses applications courantes, ainsi que ses points forts et ses limites. Cependant, nous reconnaîtrons également que les algorithmes classiques ont leurs limitations en termes de précision et de capacité à traiter des images complexes.
L'avènement du Machine Learning a apporté une avancée significative dans le domaine de la reconnaissance d'image. Cette approche permet aux ordinateurs d'apprendre à partir de données et de détecter automatiquement des motifs et des caractéristiques dans les images. Nous introduirons les principes fondamentaux du Machine Learning, tels que les ensembles de données d'apprentissage, les modèles de Machine Learning et les techniques d'entraînement et d'évaluation.
L'intégration du Machine Learning dans la reconnaissance d'image a conduit à l'utilisation de réseaux de neurones profonds, tels que les réseaux de neurones convolutifs (CNN). Ces modèles sophistiqués utilisent des architectures complexes pour extraire automatiquement des caractéristiques à partir des images, permettant ainsi une reconnaissance d'image plus précise et plus robuste.
Dans les pages suivantes, nous explorerons en détail ces différentes approches, en mettant l'accent sur les avantages, les limitations et les applications de chaque méthode. Nous plongerons dans le monde fascinant de la reconnaissance d'image et découvrirons comment elle a révolutionné de nombreux domaines. Dans cette section, nous allons explorer les algorithmes classiques qui ont été largement utilisés en reconnaissance d'image avant l'avènement du Machine Learning. Bien que ces approches aient été en partie dépassées par les avancées technologiques récentes, elles fournissent une base solide de connaissances et continuent de jouer un rôle important dans certains cas spécifiques.
Les algorithmes classiques en reconnaissance d'image sont des méthodes traditionnelles qui se concentrent sur des opérations spécifiques telles que la détection de contours, l'extraction de descripteurs, la transformation de Fourier et la correspondance de caractéristiques. Ces approches ont été développées pour extraire des informations significatives à partir d'images et sont souvent basées sur des principes mathématiques et des techniques de traitement d'image.
Bien que les algorithmes classiques aient des avantages en termes de simplicité, de rapidité et de compréhensibilité, ils présentent également des limites en termes de précision et de capacité à traiter des images complexes. Cependant, ils sont encore utilisés dans des domaines spécifiques où des connaissances spécifiques du domaine ou des contraintes de ressources sont présentes.
Dans les prochaines sections, nous explorerons plus en détail les algorithmes classiques utilisés en reconnaissance d'image, tels que la détection de contours, les descripteurs d'images, la transformation de Fourier et les méthodes de correspondance. Nous discuterons de leurs principes fondamentaux, de leurs avantages et de leurs limitations, tout en mettant en évidence leurs cas d'utilisation spécifiques.""", 20)

print(r)

end = time.time()
print(f"temps mise pour le traitement {end - start}")
