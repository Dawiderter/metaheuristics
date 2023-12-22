# Wyznaczanie parametrów
## Tabu

### Rozwiązanie początkowe losowe czy na podstawie MST?
![](plots/mst_vs_rand_opt.png)
![](plots/mst_vs_rand_time.png)

### Sąsiedztwo pełne czy losowe?
![](plots/full_vs_rand_opt.png)
![](plots/full_vs_rand_time.png)

### Długość listy
![](plots/len_list_opt.png)
![](plots/len_list_time.png)

### Werdykt
Ostatecznie wybrałem zaczynanie z rozwiązania wygenerowanego na podstawie MST, pełne sąsiedztwo i długość listy 50.

## Wyżarzanie

### Temperatura początkowa
![](plots/temp_opt.png)
![](plots/temp_time.png)

### Wielkość epoki
![](plots/epoch_opt.png)
![](plots/epoch_time.png)

### Liczba epok od ostatniej poprawy
![](plots/stagn_opt.png)
![](plots/stagn_time.png)

### Chłodzenie
![](plots/cooling_opt.png)
![](plots/cooling_time.png)

### Werdykt
Ostatecznie wybrałem parametry:
- Temperatura początkowa 1.0 * n
- Wielkość epoki 5.0 * n
- Epoki od poprawy 2.0 * n
- Chłodzenie 0.98


# Wyniki

|Test|Optymalna ścieżka|Tabu - Min cykl|Tabu - Średni cykl|Wyż - Min cykl|Wyż - Średni cykl|Local - Min cykl|Local - Średni cykl|
|------|------|------|------|------|------|------|------|
|xqf131|564|581|593.41|573|594.01|572|611.84|
|xqg237|1019|1031|1069.64|1050|1100.23|1066|1116.87|
|pma343|1368|1430|1456.03|1419|1461.35|1427|1482.43|
|pka379|1332|1365|1397.26|1370|1418.07|1398|1446.53|
|bcl380|1621|1718|1758.2|1692|1789.88|1712|1816.55|
|pbl395|1281|1340|1362.6|1350|1404.05|1346|1427.07|
|pbk411|1343|1396|1416.54|1422|1480.4|1416|1489.8|
|pbn423|1365|1407|1432.62|1446|1503.73|1454|1521.0|
|pbm436|1443|1512|1540.41|1533|1583.76|1532|1609.67|
|xql662|2513|2639|2703.01|2718|2804.33|2701|2814.83|
|xit1083|3558|3813|3938.49|3861|4000.98|3934|4017.41|
|icw1483|4416|4773|4901.45|4807|4913.64|4866|4981.9|
|djc1785|6115|6664|6775.35|6685|6834.09|6741|6871.49|
|dcb2086|6600|7255|7371.4|7202|7361.47|7344|7472.49|
|pds2566|xxx|8439|8585.2|8364|8532.12|8511|8677.17|

