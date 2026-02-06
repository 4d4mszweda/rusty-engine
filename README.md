# Rusty Engine – API do tworzenia scen 3D (Rust)

## Opis projektu

**Rusty Engine** to API napisane w **Rust**, którego celem jest szybkie budowanie i uruchamianie własnych scen 3D. Silnik udostępnia prosty interfejs do:

- układania scen (dodawanie/usuwanie obiektów),
- ładowania modeli `.obj`,
- nakładania tekstur i zmiany kolorów,
- ustawiania materiałów (matte / glossy) oraz efektów środowiskowych:
  - **reflection** (odbicia),
  - **refraction** (załamanie światła),
- transformacji obiektów (translation/rotation/scale),
- dodawania **światła**,
- ustawiania **skyboxa**,
- Rendering instancyjny

---

## Funkcjonalności API

### Scena i obiekty

- ładowanie modeli: `Mesh::from_obj("assets/models/…")`,
- tworzenie obiektów sceny (np. `SceneObject::new(...)`),
- transformacje:
  - pozycja,
  - skala,
  - obrót (również animowany),
- parametry obiektu:
  - kolor bazowy,
  - tekstura,
  - materiał (np. matowy / błyszczący),
  - tryb środowiskowy **EnvMode** (reflect / refract).

### Materiały i efekty (gotowce)

- **Matte** – powierzchnie rozpraszające światło,
- **Glossy** – powierzchnie błyszczące,
- **Reflection** – odbicia z mapy środowiska,
- **Refraction** – efekt “szkła/lodu” z parametrem IOR (index of refraction).

### Skybox

- możliwość ustawienia skyboxa jako tła sceny (mapa środowiska używana również do odbić/załamań).

### Oświetlenie

- dodawanie światła do sceny (np. światło kierunkowe/punktowe – zależnie od implementacji),
- wpływ światła na obiekty i materiały.

---

## GUI (Egui)

Silnik zawiera graficzny interfejs użytkownika oparty o **egui**, który umożliwia m.in.:

- podgląd parametrów kamery,
- debug sceny (np. liczba obiektów, parametry renderingu),
- szybkie przełączanie trybów (np. pokaz siatki / debug),
- wygodne testowanie materiałów (matte/glossy, reflect/refract).

---

## Sterowanie

- `W` – ruch do przodu
- `S` – ruch do tyłu
- `A` – ruch w lewo
- `D` – ruch w prawo
- `M` – przełączanie trybu kamery (free-look)
- `PPM` (prawy przycisk myszy) – obrót kamery (rozejrzenie) / kontrola widoku (tylko w swobodnej kamerze)
- `Esc` – wyjście z aplikacji

---

## Scena zimowa (przykładowa prezentacja)

W repozytorium znajduje się przykładowa scena **zimowa**, zbudowana w całości przy użyciu API silnika. Zawiera:

- zaśnieżony teren z powtarzaną teksturą śniegu,
- domek w zimowym klimacie,
- kilka drzew (wariacje skali i rotacji),
- zaspy i kamienie,
- bałwana złożonego z kilku sfer,
- element “lodowy” wykorzystujący **refraction** (efekt szkła/lodu),
- Rendering instancyjny “opady śniegu” (jeśli włączone w konfiguracji sceny).

---

## Obsługa błędów

Silnik ma wbudowaną obsługę błędów i walidację typowych problemów:

- brak pliku modelu `.obj` / błędy parsowania,
- brak tekstury lub niepoprawny format,
- problemy z inicjalizacją kontekstu OpenGL/GLFW,
- błędy zasobów (np. shader/tekstura/mesh) raportowane w czytelny sposób (log/debug).

Dzięki temu łatwiej diagnozować problemy podczas tworzenia nowych scen oraz dodawania assetów.

---

## Użyte biblioteki (crates)

Projekt korzysta z następujących zależności:

- `gl` – bindingi OpenGL (rendering)
- `glfw` – okno, kontekst, input
- `tobj` – ładowanie modeli `.obj`
- `cgmath` – macierze i wektory (transformacje, kamera)
- `image` – wczytywanie tekstur
- `num` – narzędzia numeryczne / pomocnicze typy
- `egui` – GUI
- `egui_glow` – integracja egui z OpenGL
- `rand` – losowość (np. wariacje w scenie / particles)

---

## Jak uruchomić

### Wymagania

- **Rust** (toolchain stable)
- środowisko z obsługą **OpenGL**
- na Linuxie mogą być potrzebne pakiety GLFW (np. `libglfw3-dev`) oraz sterowniki OpenGL
- dodatkowe pakiety wymagane przez biblioteki (przy próbie kompilacji zostaną wypisane)
- katalog `assets/` z modelami i teksturami (w repozytorium / obok binarki)

### Build & Run

```bash
cargo run --release
```
