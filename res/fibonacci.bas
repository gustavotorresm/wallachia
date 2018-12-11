10 INPUT "How many numbers should i generate"; N
20 PRINT "Generating "; N; " numbers..."
30 DIM R (N)
40 FOR I = 0 TO N
50 IF I <= 1 THEN 100
60 LET R(I) = R(I-1) + R(I-2)
65 PRINT R(I)
70 NEXT
80 GOTO 200

100 LET R(I) = 1
110 GOTO 65

200 END
