140 END
100 IF LEN(A$) = 0 THEN GOTO 90
120 IF A$ = "Y" OR A$ = "y" THEN GOTO 30
40 S$ = ""
60 S$ = S$ + "*"
30 INPUT "How many stars do you want: "; N
70 NEXT I
90 INPUT "Do you want more stars? "; A$
80 PRINT S$
130 PRINT "Goodbye "; U$
50 FOR I = 1 TO N
20 PRINT "Hello "; U$
110 A$ = LEFT$(A$, 1)
10 INPUT "What is your name: "; U$
