---
description: General rules
auto_execution_mode: 1
---

🧭 Ogólne wytyczne
Zawsze działaj zgodnie z systemem Windows 11 i terminalem powershell.

Zawsze odpowiedaj w języku polskim

Odpowiadaj zwięźle, ale wyczerpująco.

Rozwiązuj problemy krok po kroku aż do skutku, stosując najlepsze praktyki i światowe standardy.

W przypadku błędu – wróć, popraw i spróbuj ponownie.

Nie popełniaj błędów i rób wszystko dokładnie.

Jeżeli twoje narzędzia do pracy z dokumentami zawodzą, użyj mcp server filesystem

Nigdy nie baw się w odgadywanie zawartości plików, jeżeli masz problem z odczytaniem zawartości plików, wykorzystaj dostanie się do jego zawartości po jawnie zdefiniowanym zakresie linii, jeżeli to nie pomaga, spróbuj użyć innych narzędzi. Jeżeli nadal nie ma możliwości uzyskania dostępu, nie wymyślaj i strzelaj co jest w danym pliku tylko poproś o jego zawartość

🛠️ Kod i implementacja
ZAWSZE stosuj się do zasad z workflow o nazwie trudne-zadania (o opisie Workflow tłumaczące cascadzie w jaki sposób ma rozwiązywać zadania)

Przestrzegaj zasady SOLID, szczególnie Single Responsibility – małe pliki, jasny podział, czytelność, łatwa diagnostyka.

Analizuj cały projekt przed każdą zmianą – dopasuj się do istniejącej struktury i stylu.

Dbaj o spójność kodu, komponentów i stylów – trzymaj się konwencji i centralnych styli.

Stosuj nowoczesne wzorce, czysty kod i przygotowanie na przyszłą rozbudowę.

Na koniec zmian przejrzyj kod pod kątem potencjalnych błędów i od razu je popraw.

Przed zakończeniem zadania, kiedy już wszystko jest zrobione, wypisz wszystko co zostało zrobione i sprawdź czy pokrywa się to z listą zadań które zostały wypisane na początku zadania. Jeżeli czegoś nie ma, zrób to

Po każdej zmianie czy stworzeniu nowego pliku sprawdź czy są w nim jakiekolwiek błedy, jeżeli są, rozwiąż je

Jeśli coś przestaje być używane – usuń niepotrzebne pliki.

🗂️ Dokumentacja
Twórz ją w folderze docs (chyba że inna zasada stanowi inaczej - przykładowo zasada tworzenia dokumentacji propozycji testów) – pełna, szczegółowa, ale przystępna.

Pisz prostym językiem, z przykładami, porównaniami i wyjaśnieniami nawet dla osób spoza branży.

Skup się na użyteczności dokumentacji – ma pomagać zrozumieć i używać funkcjonalności.

🧱 Organizacja pracy
Zanim coś zrobisz: przeanalizuj temat, projekt, zaplanuj działania, rozpisz etapy.

Pracuj małymi krokami, dokładnie i bez uproszczeń.

Na początku twórz wszystkie potrzebne foldery, jeżeli zadanie ich wymaga.

Zmiany w kodzie zawsze zaczynaj od przygotowania pustych plików.

Kiedy uruchamiasz komendy, masz nie tylko sprawdzć czy przypadkiem nie wyszły z błędem (np. exit 1) ale wogóle masz poczekać na to, aż komendą się zakończy i definitywnie zakończy działanie - dopiero wtedy masz analizować jej output i podejmować odpowiednie akcje. Czekaj na pełne zakończenie uruchomionego procesu

Jeśli masz problem z uruchomieniem komendy, spróbuj uruchomić ją z poziomu terminala, a nie z poziomu Windsurfa. Jeżeli komenda nie odpowiada, lub pojawia się problem z uruchomieniem, zaniechaj tej akcji i spróbuj jeszcze raz - aż zobaczysz, że na pewno zadziałało

Po każdych zmianach w kodzie, zanim uznasz zadanie za wykonane, zawsze sprawdź czy nie ma w kodzie błędów, jeżeli jest, napraw je. Do szukania błędów w kodzie wykorzystaj komendy "npm run type-check" oraz "npx expo-doctor". Jeżeli po poczekaniu na odpowiedź dowolna komenda zwraca jakikolwiek błąd, napraw go. Możesz zadanie uznać za zakończone jak nie ma już żadnych błędów po ich uruchomieniu

Przestrzegaj zasad SOLID, KISS, DRY, YAGNI, Prawa Demeter podczas pracy z kodem

Dokonuj tylko tych zmian w kodzie które są absolutnie niezbędne do wykonania zadania. Nie wolno tworzyć ani przede wszystkim edytować plików/funkcji które nie są związane z rozwiązaniem aktualnego zadania. Masz robić tylko to co absolutnie konieczne i dokładnie to o co ustaliliśmy razem

Jeżeli chcesz dostać się do zawartości pliku jawnie typuj zakres linii z dużym marginesem np. "1-300" tak by nigdy nie było sytuacji że nie przeczytasz końcówki pliku

Przy instalowaniu pakietów oraz używając komendy type-check korzystaj z npm, a nie yarn!!!

Opisy testów pisz w języku angielskim ("should...")