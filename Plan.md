  - Du organisierst Dir eine Ear-Clipping-Implementierung Deiner
Wahl. Entweder Du schreibst sie selbst, Du nimmst eine von meinen
oder Du schaust nach öffentlich verfügbarem Code (ggf. Vorsicht mit
der Lizenz). Wichtig wäre mir nur, dass sie komplexe Multipolygone
unterstützt (geschachtelte äußere Konturen / Löcher, Überlappung von
Knoten / Kanten etc.).
  - Du modifizierst die Implementierung (z.B. über Callbacks), sodass
man schrittweise durch den Algorithmus "steppen" kann und der
aktuelle State zugänglich ist.
  - Darüber implementierst Du eine Visualisierung (Window / Browser /
App / ...) mit der Möglichkeit, ein Multipolygon vorzugeben, und
mithilfe entsprechender Controls zu navigieren (vorwärs, rückwärs
steppen etc.). Später wäre es cool, wenn sogar eine gewisse
Interaktion möglich wäre, also z.B. der Benutzer das nächste Dreieck
durch Klicken / Touch wählen könnte.
  - Und was wir uns als Erweiterung überlegt haben: Im
Ear-Clipping-Algorithmus ist nicht festgelegt, welches Dreieck als
nächstes geclippt wird, wenn mehrere zur Auswahl stehen. Auf diese
Weise kann man mit unterschiedlichen Heuristiken (oder
Nutzerinteraktionen) für ein Polygon unterschiedliche
Triangulierungen erzeugen. Auf einer Triangulierung kann man (auch
auf unterschiedliche Weisen) Metriken definieren, die angeben, wie
"gelungen" sie ist. Z.B. wäre es wünschenswert, dass die Dreiecke
möglichst gleichseitig sind und man schmale Dreiecke ("slivers")
vermeidet. Auf diese Weise könnte man nach möglichst guten
Heuristiken suchen oder das Ganze sogar für den Nutzer gamifizieren
(aka. "Finde die beste Triangulierung für das gegebene Polygon") und
daraus wiederum neue Heuristiken ableiten.
  - Im schriftlichen Teil erläuterst Du die Hintergründe des
Ear-Clipping-Algorithmus selbst, beschreibst mögliche Anwendungen,
gehst auf die Heuristiken ein und erklärst Deine Visualisierung und
deren mögliche Einsätze.