# Projekthandbuch

## Dokumentation

Eine Aufnahme des Backlogs erfolgt mit Zeitstempeln.
Die zu verwendende Sprache dafür ist **Latex**.
Das das finale Log soll als **PDF** verfügbar.

Format:
```latex
\textbf{Ticketüberschrift}

\textit{Bearbeitungsbeginn:} ...
\textit{Berabeitungsende:} ...

\textit{Anmerkungen:} ...

...Ticketbeschreibung...
```

## Allgemeiner Projektablauf

### Recherche zu Technologien

Alle Rechercheergebnisse und Diskussionen zu in Frage kommenden Technologien
werden in dem Verzeichnis `concepts` gespeichert.
Die einzelnen Dokumente werden in Markdown geschrieben und haben sonst keine formatlichen Vorgaben.

### Ticketing

Für jede Aufgabe wird ein Ticket im Git Project angelegt.
Der Status des Tickets wird für die folgenden Stati aktuell gehalten:

- TODO
- In Bearbeitung
- Im Review
- Done

## Branching

Für jedes Feature wird ein Branch erstellt.
Das Format für den Branchnamen ist:
```
Feature/ISSUE<ISSUENUMBER>-<Featurename>
```

Die einzelnen Featurebranches können zu z.B. Testzwecken frei ausgebrancht werden.
Diese kleinen Branches können ohne Pullrequests auf ihren entsprechenden Featurebranch zurückgemergt werden.

Crossbranching zwischen Featurebranches soll nicht stattfinden.

Für jeden Merge auf `main` soll eine Pullrequest mit Review eines Zweiten stattfinden.
Es soll kein Rebase auf `main` stattfinden.
Für die Arbeit auf den einzelnen Featurebranches bleibt das dem Entwickler freigestellt.

## Testing

Alle Testlogs sollen in dem Unterverzeichnis `test_logs` gespeichert werden.
Diese werden nicht auf Git gepusht.
Für einen aufgetretenen Fehler soll eine Git-Issue erstellt werden.
Diese hat das Format:

```
Titel: Fehler-Code

Beschreibung des Fehlers

\```
Stackstrace
\```

```

### Unittest

Unittestlogs sollen ein dem Unterverzeichnis `test_logs/unittests` gespeichert werden.
Am Ende des Projekts sollen alle Unittestlogs in einem Commit auf Git gepusht werden.

Unittests können können direkt mit ihrem Source-Code dokumentiert werden.
Der Output von Unittests soll gelogt und gespeichert werden.

### Manuelle Tests

Manuelle Testlogs sollen ein dem Unterverzeichnis `test_logs/manual` gespeichert werden.
Am Ende des Projekts sollen alle Unittestlogs in einem Commit auf Git gepusht werden.
Manuelle Tests haben folgendes Format:
```markdown
# Testname_Testdatum

## Textumfang
... Was wurde getestet? ...

## Testerfolg
... Welche Fehler wurden gefunden, was hat funktioniert? ...
```

## Logging

Logging should work with the following Levels:

- Info
- Trace
- Warn

## Scrum

Sprintdauer: *1*
Sprint-Meeting: *Montags, nach letzer Vorlesung*
