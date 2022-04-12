
# O CPS (Constraint Satisfaction Problem)

Przypisanie jest:
- spójne (consistent) jeśli nie jest sprzeczne, z jakimkolwiek z więzów
- zupełne (complete) jeśli wszystkie zmienne mają wartość
- częściowe (partial) jeśli nie wszystkie zmienne mają wartość.

Rozwiązaniem CSP jest spójne i zupełne przypisanie (wartość każdej zmienna jest ok).

Rozwiązywanie problemu satysfakcji więzów, polega na szukaniu takiego przypisania (assignment) wartości dla zmiennych,
które nie jest sprzeczne z jakimkolwiek z więzów, takie przypisanie nazywa się spójnym (consistent or legal)

Zaletą formułowania problemu jako CPS jest ogólność rozwiązania. Jeśli mamy system rozwiązujący CSP łatwiej 
jest nam go wykorzystać do rozwiązania innych problemów za jego pomocą, niż wymyślać każdorazowo specyficzne rozwiązanie 
używające innej techniki przeszukiwania.
Dodatkowo CPS może być szybsze niż inne techniki ponieważ, może szybko eliminować spore części przestrzeni stanów.

```
function BACKTRACK(assignment, CSP):
    if assignment is complete
```


# Propagacja więzów: Inferencja w CSP

Inference - wnioskowanie.

W CSP wnioskowanie to propagacja więzów: jest to użycie więzów do redukcji ilości dopuszczalnych wartości dla danej zmiennej.
Taka redukcja, może z kolei wpłynąć na ilość dopuszczalnych wartości dla innych zmiennych.

Zmienne dla CSP z węzłami binarnymi i unarnymi można przedstawiać jako wierzchołki w grafie, gdzie krawędzie to węzły binarne. 
W interpretacji CPS z binarnymi więzami pojawia się pojęcie spójności lokalnej (local cosistency).
Wyróżnia się jej następujące typy:

1. Spójność węzłowa (node consistency)
   Pojedyncza zmienna jest węzłowo spójna, jeśli wszystkie wartości z dziedziny zmiennej spełniają jej wszystkie więzy unarne.
   Cały graf jest węzłowo spójny jeśli wszystkie wierzchołki w grafie są węzłowo spójne.

2. Spójność łukowa (arc consistency)
   Zmienna jest łukowo spójna jeśli każda wartość w jej dziedzinie spełnia wszystkie jej więzy binarne.
   Formalnie zmienna Xi jest łukowo spójna względem innej zmiennej Xj jeśli dla każdej wartości w aktualnej dziedzinie Di jest jakaś wartość w Dj
   taka, która spełnia węzeł binarny na krawędzi (Xi, Xj). Ponownie sieć / graf (network) jest łukowo spójna (arc-consistent) jeśli każda
   zmienna jest łukowo spójna z każdą inną zmienną.

   ## AC-3
   
   Najpopularniejszym algorytmem na spójnośc łukową jest AC-3

   ```
   function AC-3(CSP) zwraca false jeśli wykryto brak spójności true wpp.
       inputs: csp, binarny csp
   
       while queue is not empty do
           (Xi, Xj) <- Remove-First(queue)
           If Revise(cps, Xi, Xj) then
               if size of Di == 0 then return false
               for each Xk in Xi.Neighbors - {Xj} do
                   add (Xk, Xi) to queue
           return true

   // Modyfikujemy dziedzinę Xi tak, żeby byla łukowo spójna z Xj
   // Zwraca true jesli usunęliśmy choć jedną zmienną z Di false wpp.
   function Revise(csp, Xi, Xj) return ture iff we revise the domain of Xi
       revised <- false
       for each x in Di do
           if no value y in Dj allows (x, y) to satify the constraint between Xi and Xj then
               delete x from Di
               revised <- true
       return revised
   ```
3. Path consistency
   Mocniejszy rodzaj spójności niż łukowa, która patrzy na zmiennej trójkami.
   A two variable set {Xi, Xj} to path consistent with respect to a third variable Xm if
   for every assignment consistent with the constraint on {Xi, Xj}, there is an assignment to Xm
   that satisfies the constaints on {Xi, Xm} and {Xj, Xm}.
   Sprawdzamy, czy spośród wszystkich przypisań, które spełniają Xi, Xj czy istnieje przypisanie
4. K-consistency
   Uogólnienie powyższych spójności na k zmiennych.
   Żeby K-spójność zachodziła: for any set of k - 1 variables and for any consistent assignment to
   those variables, a consistent value can allways be assigned to any kth variable.
   1-consistency 
5. Global constraints
   Ograniczenia, które dotyczą arbitralnie wielu zmiennych. np. alldiff.
   Proces sprawdzania polega na usuwaniu wszystkich zmiennych, które mają singleton jako dziedzinę
   i usuwanie tej wartości z dziedzin pozostałych zmiennych.
   Inconsistency wystąpi jeśli łączna ilośc zmiennych dostępna w dziedzinach wszystkich zmiennych, jest mniejsza
   niż ilośc zmiennych - wówczas wiemy, że nie da się przydzielić tak wartości, żeby żadne się nie powtórzyły.
6. Resource constraint
   czasem nazywane też almost constraint   

Dla dużych problemów typu resource limited o dziedzinach całkowitych,
przechowywanie zbioru wartości dla każdej zmiennej jest niepraktyczne.
Zamiast tego operuje się na górnych i dolnych ograniczeniach i problem rozwiązywany jest przez
bound propagation. Np. przy airline scheduling'u mamy dwa loty F1 oraz F2 o pojemnościach wynoszących odpowiednio
do 165 i do 385 -> dziedziny [0, 165] oraz [0, 385]. Dodatkowo mamy constraint, że oba loty muszą przewieść nie mniej niż 420 pasażerów.
Porpagując bound constraint otrzymamy następujące dziedziny [35, 165] oraz [255, 385].
Mówimy, że CSP jest bound consistent jeśli dla każdej zmiennej istnieje wartość, która spełnia ograniczenia góre i dolne dla tej zmiennej.


# Backtracking search for CSP

(pol) Przeszukiwanie z nawrotami, jest to wariant przeszukiwania w głąb, w którym stanem jest **niepełne** podstawienie zmiennych.
Akcją jest wykonanie jakiegoś podstawienia. Dodatkowo w akcji przeprowadzamy jakieś wnioskowanie - dodajemy, usuwamy więzy modyfikujemy dziedziny?

Dodatkowo jesteśmy się w stanie cofnąć do stanu wcześniejszego sprzed rozumowania.

