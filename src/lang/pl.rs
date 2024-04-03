lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Twój pulpit"),
        ("desk_tip", "Aby połączyć się z tym urządzeniem, użyj poniższego ID i hasła"),
        ("Password", "Hasło"),
        ("Ready", "Gotowe"),
        ("Established", "Nawiązano"),
        ("connecting_status", "Łączenie"),
        ("Enable service", "Włącz usługę"),
        ("Start service", "Uruchom usługę"),
        ("Service is running", "Usługa uruchomiona"),
        ("Service is not running", "Usługa nie jest uruchomiona"),
        ("not_ready_status", "Brak gotowości"),
        ("Control Remote Desktop", "Połącz się z"),
        ("Transfer file", "Transfer plików"),
        ("Connect", "Połącz"),
        ("Recent sessions", "Ostatnie sesje"),
        ("Address book", "Książka adresowa"),
        ("Confirmation", "Potwierdzenie"),
        ("TCP tunneling", "Tunelowanie TCP"),
        ("Remove", "Usuń"),
        ("Refresh random password", "Odśwież losowe hasło"),
        ("Set your own password", "Ustaw własne hasło"),
        ("Enable keyboard/mouse", "Włącz klawiaturę/mysz"),
        ("Enable clipboard", "Włącz schowek"),
        ("Enable file transfer", "Włącz transfer pliku"),
        ("Enable TCP tunneling", "Włącz tunelowanie TCP"),
        ("IP Whitelisting", "Biała lista IP"),
        ("ID/Relay Server", "Serwer ID/Pośredniczący"),
        ("Import server config", "Importuj konfigurację serwera"),
        ("Export Server Config", "Eksportuj konfigurację serwera"),
        ("Import server configuration successfully", "Import konfiguracji serwera zakończono pomyślnie"),
        ("Export server configuration successfully", "Eksport konfiguracji serwera zakończono pomyślnie"),
        ("Invalid server configuration", "Nieprawidłowa konfiguracja serwera"),
        ("Clipboard is empty", "Schowek jest pusty"),
        ("Stop service", "Zatrzymaj usługę"),
        ("Change ID", "Zmień ID"),
        ("Your new ID", "Twój nowy ID"),
        ("length %min% to %max%", "o długości od %min% do %max%"),
        ("starts with a letter", "rozpoczyna się literą"),
        ("allowed characters", "dozwolone znaki"),
        ("id_change_tip", "Nowy ID może być złożony z małych i dużych liter a-zA-z, cyfry 0-9 oraz _ (podkreślenie). Pierwszym znakiem powinna być litera a-zA-Z, a całe ID powinno składać się z 6 do 16 znaków."),
        ("Website", "Strona internetowa"),
        ("About", "O aplikacji"),
        ("Slogan_tip", "Tworzone z miłością w tym pełnym chaosu świecie!"),
        ("Privacy Statement", "Oświadczenie o ochronie prywatności"),
        ("Mute", "Wycisz"),
        ("Build Date", "Zbudowano"),
        ("Version", "Wersja"),
        ("Home", "Pulpit"),
        ("Audio Input", "Wejście audio"),
        ("Enhancements", "Ulepszenia"),
        ("Hardware Codec", "Kodek sprzętowy"),
        ("Adaptive bitrate", "Adaptacyjny bitrate"),
        ("ID Server", "Serwer ID"),
        ("Relay Server", "Serwer pośredniczący"),
        ("API Server", "Serwer API"),
        ("invalid_http", "Nieprawidłowe żądanie http"),
        ("Invalid IP", "Nieprawidłowe IP"),
        ("Invalid format", "Nieprawidłowy format"),
        ("server_not_support", "Serwer nie obsługuje tej funkcji"),
        ("Not available", "Niedostępne"),
        ("Too frequent", "Zbyt często"),
        ("Cancel", "Anuluj"),
        ("Skip", "Pomiń"),
        ("Close", "Zamknij"),
        ("Retry", "Ponów"),
        ("OK", "OK"),
        ("Password Required", "Wymagane jest hasło"),
        ("Please enter your password", "Wpisz proszę twoje hasło"),
        ("Remember password", "Zapamiętaj hasło"),
        ("Wrong Password", "Błędne hasło"),
        ("Do you want to enter again?", "Czy chcesz wprowadzić ponownie?"),
        ("Connection Error", "Błąd połączenia"),
        ("Error", "Błąd"),
        ("Reset by the peer", "Połączenie zresetowanie przez zdalne urządzenie"),
        ("Connecting...", "Łączenie..."),
        ("Connection in progress. Please wait.", "Trwa łączenie. Proszę czekać."),
        ("Please try 1 minute later", "Spróbuj za minutę"),
        ("Login Error", "Błąd logowania"),
        ("Successful", "Sukces"),
        ("Connected, waiting for image...", "Połączono, oczekiwanie na obraz..."),
        ("Name", "Nazwa"),
        ("Type", "Typ"),
        ("Modified", "Zmodyfikowany"),
        ("Size", "Rozmiar"),
        ("Show Hidden Files", "Pokaż ukryte pliki"),
        ("Receive", "Pobierz"),
        ("Send", "Wyślij"),
        ("Refresh File", "Odśwież plik"),
        ("Local", "Lokalny"),
        ("Remote", "Zdalny"),
        ("Remote Computer", "Komputer zdalny"),
        ("Local Computer", "Komputer lokalny"),
        ("Confirm Delete", "Potwierdź usunięcie"),
        ("Delete", "Usuń"),
        ("Properties", "Właściwości"),
        ("Multi Select", "Wielokrotny wybór"),
        ("Select All", "Zaznacz wszystko"),
        ("Unselect All", "Odznacz wszystko"),
        ("Empty Directory", "Pusty katalog"),
        ("Not an empty directory", "Katalog nie jest pusty"),
        ("Are you sure you want to delete this file?", "Czy na pewno chcesz usunąć ten plik?"),
        ("Are you sure you want to delete this empty directory?", "Czy na pewno chcesz usunąć ten pusty katalog?"),
        ("Are you sure you want to delete the file of this directory?", "Czy na pewno chcesz usunąć pliki z tego katalogu?"),
        ("Do this for all conflicts", "wykonaj dla wszystkich konfliktów"),
        ("This is irreversible!", "To jest nieodwracalne!"),
        ("Deleting", "Usuwanie"),
        ("files", "pliki"),
        ("Waiting", "Oczekiwanie"),
        ("Finished", "Zakończono"),
        ("Speed", "Prędkość"),
        ("Custom Image Quality", "Niestandardowa jakość obrazu"),
        ("Privacy mode", "Tryb prywatny"),
        ("Block user input", "Blokuj peryferia użytkownika"),
        ("Unblock user input", "Odblokuj peryferia użytkownika"),
        ("Adjust Window", "Dostosuj okno"),
        ("Original", "Oryginalny"),
        ("Shrink", "Zmniejsz"),
        ("Stretch", "Rozciągnij"),
        ("Scrollbar", "Pasek przewijania"),
        ("ScrollAuto", "Przewijanie automatyczne"),
        ("Good image quality", "Dobra jakość obrazu"),
        ("Balanced", "Zrównoważony"),
        ("Optimize reaction time", "Zoptymalizuj czas reakcji"),
        ("Custom", "Niestandardowe"),
        ("Show remote cursor", "Pokazuj zdalny kursor"),
        ("Show quality monitor", "Parametry połączenia"),
        ("Disable clipboard", "Wyłącz schowek"),
        ("Lock after session end", "Zablokuj po zakończeniu sesji"),
        ("Insert", "Wyślij"),
        ("Insert Lock", "Zablokuj zdalne urządzenie"),
        ("Refresh", "Odśwież"),
        ("ID does not exist", "ID nie istnieje"),
        ("Failed to connect to rendezvous server", "Nie udało się połączyć z serwerem połączeń"),
        ("Please try later", "Spróbuj później"),
        ("Remote desktop is offline", "Zdalny pulpit jest offline"),
        ("Key mismatch", "Niezgodność klucza"),
        ("Timeout", "Przekroczono czas oczekiwania"),
        ("Failed to connect to relay server", "Nie udało się połączyć z serwerem pośredniczącym"),
        ("Failed to connect via rendezvous server", "Nie udało się połączyć przez serwer połączeń"),
        ("Failed to connect via relay server", "Nie udało się połączyć przez serwer pośredniczący"),
        ("Failed to make direct connection to remote desktop", "Nie udało się nawiązać bezpośredniego połączenia z pulpitem zdalnym"),
        ("Set Password", "Ustaw hasło"),
        ("OS Password", "Hasło systemu operacyjnego"),
        ("install_tip", "RustDesk może nie działać poprawnie na maszynie zdalnej z przyczyn związanych z UAC. W celu uniknięcia problemów z UAC, kliknij poniższy przycisk by zainstalować RustDesk w swoim systemie."),
        ("Click to upgrade", "Zaktualizuj"),
        ("Click to download", "Pobierz"),
        ("Click to update", "Uaktualnij"),
        ("Configure", "Konfiguruj"),
        ("config_acc", "Konfiguracja konta"),
        ("config_screen", "Konfiguracja ekranu"),
        ("Installing ...", "Instalowanie..."),
        ("Install", "Zainstaluj"),
        ("Installation", "Instalacja"),
        ("Installation Path", "Ścieżka instalacji"),
        ("Create start menu shortcuts", "Utwórz skrót w menu"),
        ("Create desktop icon", "Utwórz skrót na pulpicie"),
        ("agreement_tip", "Wskazówki do umowy/zgody"),
        ("Accept and Install", "Akceptuj i zainstaluj"),
        ("End-user license agreement", "Umowa licencyjna użytkownika końcowego"),
        ("Generating ...", "Trwa generowanie..."),
        ("Your installation is lower version.", "Twoja instalacja jest w niższej wersji"),
        ("not_close_tcp_tip", "Podczas korzystanie z tunelowania, nie zamykaj tego okna."),
        ("Listening ...", "Nasłuchiwanie..."),
        ("Remote Host", "Host zdalny"),
        ("Remote Port", "Port zdalny"),
        ("Action", "Akcja"),
        ("Add", "Dodaj"),
        ("Local Port", "Lokalny port"),
        ("Local Address", "Lokalny adres"),
        ("Change Local Port", "Zmień lokalny port"),
        ("setup_server_tip", "W celu uzyskania szybszego połączenia, skorzystaj z własnego serwera połączeń."),
        ("Too short, at least 6 characters.", "Za krótkie, min. 6 znaków"),
        ("The confirmation is not identical.", "Potwierdzenie nie jest identyczne."),
        ("Permissions", "Uprawnienia"),
        ("Accept", "Akceptuj"),
        ("Dismiss", "Odrzuć"),
        ("Disconnect", "Rozłącz"),
        ("Enable file copy and paste", "Zezwalaj na kopiowanie i wklejanie plików"),
        ("Connected", "Połączony"),
        ("Direct and encrypted connection", "Połączenie bezpośrednie i szyfrowane"),
        ("Relayed and encrypted connection", "Połączenie pośrednie i szyfrowane"),
        ("Direct and unencrypted connection", "Połączenie bezpośrednie i nieszyfrowane"),
        ("Relayed and unencrypted connection", "Połączenie pośrednie i nieszyfrowane"),
        ("Enter Remote ID", "Wprowadź zdalne ID"),
        ("Enter your password", "Wprowadź hasło"),
        ("Logging in...", "Trwa logowanie..."),
        ("Enable RDP session sharing", "Włącz udostępnianie sesji RDP"),
        ("Auto Login", "Automatyczne logowanie"),
        ("Enable direct IP access", "Włącz bezpośredni dostęp IP"),
        ("Rename", "Zmień nazwę"),
        ("Space", "Przestrzeń"),
        ("Create desktop shortcut", "Utwórz skrót na pulpicie"),
        ("Change Path", "Zmień ścieżkę"),
        ("Create Folder", "Utwórz folder"),
        ("Please enter the folder name", "Wpisz nazwę folderu"),
        ("Fix it", "Napraw to"),
        ("Warning", "Ostrzeżenie"),
        ("Login screen using Wayland is not supported", "Ekran logowania korzystający z Wayland nie jest obsługiwany"),
        ("Reboot required", "Wymagany ponowne uruchomienie"),
        ("Unsupported display server", "Nieobsługiwany serwer wyświetlania"),
        ("x11 expected", "Wymagany jest X11"),
        ("Port", "Port"),
        ("Settings", "Ustawienia"),
        ("Username", "Nazwa użytkownika"),
        ("Invalid port", "Nieprawidłowy port"),
        ("Closed manually by the peer", "Połączenie zakończone ręcznie przez zdalne urządzenie"),
        ("Enable remote configuration modification", "Włącz zdalną modyfikację konfiguracji"),
        ("Run without install", "Uruchom bez instalacji"),
        ("Connect via relay", "Połącz bezpośrednio"),
        ("Always connect via relay", "Zawsze łącz pośrednio"),
        ("whitelist_tip", "Zezwalaj na łączenie z tym komputerem tylko z adresów IP znajdujących się na białej liście"),
        ("Login", "Zaloguj"),
        ("Verify", "Zweryfikuj"),
        ("Remember me", "Zapamiętaj mnie"),
        ("Trust this device", "Dodaj to urządzenie do zaufanych"),
        ("Verification code", "Kod weryfikacyjny"),
        ("verification_tip", "Nastąpiło logowanie z nowego urządzenia, kod weryfikacyjny został wysłany na podany adres email, wprowadź kod by kontynuować proces logowania"),
        ("Logout", "Wyloguj"),
        ("Tags", "Tagi"),
        ("Search ID", "Szukaj ID"),
        ("whitelist_sep", "Oddzielone przecinkiem, średnikiem, spacją lub w nowej linii"),
        ("Add ID", "Dodaj ID"),
        ("Add Tag", "Dodaj Tag"),
        ("Unselect all tags", "Odznacz wszystkie tagi"),
        ("Network error", "Błąd sieci"),
        ("Username missed", "Nieprawidłowe nazwa użytkownika"),
        ("Password missed", "Nieprawidłowe hasło"),
        ("Wrong credentials", "Błędne dane uwierzytelniające"),
        ("The verification code is incorrect or has expired", "Kod weryfikacyjny jest niepoprawny lub wygasł"),
        ("Edit Tag", "Edytuj tag"),
        ("Forget Password", "Zapomnij hasło"),
        ("Favorites", "Ulubione"),
        ("Add to Favorites", "Dodaj do ulubionych"),
        ("Remove from Favorites", "Usuń z ulubionych"),
        ("Empty", "Pusto"),
        ("Invalid folder name", "Nieprawidłowa nazwa folderu"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Socks5/Http(s) Proxy", "Proxy Socks5/Http(s)"),
        ("Discovered", "Wykryte"),
        ("install_daemon_tip", "By uruchomić RustDesk przy starcie systemu, musisz zainstalować usługę systemową."),
        ("Remote ID", "Zdalne ID"),
        ("Paste", "Wklej"),
        ("Paste here?", "Wkleić tutaj?"),
        ("Are you sure to close the connection?", "Czy na pewno chcesz zakończyć połączenie?"),
        ("Download new version", "Pobierz nową wersję"),
        ("Touch mode", "Tryb dotykowy"),
        ("Mouse mode", "Tryb myszy"),
        ("One-Finger Tap", "Dotknij jednym palcem"),
        ("Left Mouse", "Lewy klik myszy"),
        ("One-Long Tap", "Przytrzymaj jednym palcem"),
        ("Two-Finger Tap", "Dotknij dwoma palcami"),
        ("Right Mouse", "Prawy przycisk myszy"),
        ("One-Finger Move", "Przesuń jednym palcem"),
        ("Double Tap & Move", "Dotknij dwukrotnie i przesuń"),
        ("Mouse Drag", "Przeciągnij myszą"),
        ("Three-Finger vertically", "Trzy palce pionowo"),
        ("Mouse Wheel", "Kółko myszy"),
        ("Two-Finger Move", "Ruch dwoma palcami"),
        ("Canvas Move", "Ruch ekranu"),
        ("Pinch to Zoom", "Uszczypnij, aby powiększyć"),
        ("Canvas Zoom", "Powiększanie ekranu"),
        ("Reset canvas", "Reset ekranu"),
        ("No permission of file transfer", "Brak uprawnień na przesyłanie plików"),
        ("Note", "Notatka"),
        ("Connection", "Połączenie"),
        ("Share Screen", "Udostępnij ekran"),
        ("Chat", "Czat"),
        ("Total", "Łącznie"),
        ("items", "elementów"),
        ("Selected", "zaznaczonych"),
        ("Screen Capture", "Przechwytywanie ekranu"),
        ("Input Control", "Kontrola wejścia"),
        ("Audio Capture", "Przechwytywanie dźwięku"),
        ("File Connection", "Przekazywanie plików"),
        ("Screen Connection", "Przekazywanie ekranu"),
        ("Do you accept?", "Akceptujesz?"),
        ("Open System Setting", "Otwórz ustawienia systemowe"),
        ("How to get Android input permission?", "Jak uzyskać uprawnienia do wprowadzania danych w systemie Android?"),
        ("android_input_permission_tip1", "Aby można było sterować Twoim urządzeniem za pomocą myszy lub dotyku, musisz zezwolić RustDesk na korzystanie z usługi \"Ułatwienia dostępu\"."),
        ("android_input_permission_tip2", "Przejdź do następnej strony ustawień systemowych, znajdź i wejdź w [Zainstalowane usługi], włącz usługę [RustDesk Input]."),
        ("android_new_connection_tip", "Otrzymano nowe żądanie zdalnego dostępu, które chce przejąć kontrolę nad Twoim urządzeniem."),
        ("android_service_will_start_tip", "Włączenie opcji „Przechwytywanie ekranu” spowoduje automatyczne uruchomienie usługi, umożliwiając innym urządzeniom żądanie połączenia z Twoim urządzeniem."),
        ("android_stop_service_tip", "Zamknięcie usługi spowoduje automatyczne zamknięcie wszystkich nawiązanych połączeń."),
        ("android_version_audio_tip", "Bieżąca wersja systemu Android nie obsługuje przechwytywania dźwięku, zaktualizuj system do wersji Android 10 lub nowszej."),
        ("android_start_service_tip", "Kliknij [Uruchom serwis] lub włącz uprawnienia [Zrzuty ekranu], aby uruchomić usługę udostępniania ekranu."),
        ("android_permission_may_not_change_tip", "Uprawnienia do nawiązanych połączeń nie mogą być zmieniane automatycznie, dopiero po ponownym połączeniu."),
        ("Account", "Konto"),
        ("Overwrite", "Nadpisz"),
        ("This file exists, skip or overwrite this file?", "Ten plik istnieje, pominąć czy nadpisać ten plik?"),
        ("Quit", "Zrezygnuj"),
        ("Help", "Pomoc"),
        ("Failed", "Niepowodzenie"),
        ("Succeeded", "Udało się"),
        ("Someone turns on privacy mode, exit", "Ktoś włącza tryb prywatności, wyjdź"),
        ("Unsupported", "Niewspierane"),
        ("Peer denied", "Odmowa dostępu"),
        ("Please install plugins", "Zainstaluj wtyczkę"),
        ("Peer exit", "Wyjście ze zdalnego urządzenia"),
        ("Failed to turn off", "Nie udało się wyłączyć"),
        ("Turned off", "Wyłączony"),
        ("Language", "Język"),
        ("Keep RustDesk background service", "Zachowaj usługę RustDesk w tle"),
        ("Ignore Battery Optimizations", "Ignoruj optymalizację baterii"),
        ("android_open_battery_optimizations_tip", "Jeśli chcesz wyłączyć tę funkcję, przejdź do następnej strony ustawień aplikacji RustDesk, znajdź i wprowadź [Bateria], odznacz [Bez ograniczeń]"),
        ("Start on boot", "Autostart"),
        ("Start the screen sharing service on boot, requires special permissions", "Uruchom usługę udostępniania ekranu podczas startu, wymaga specjalnych uprawnień"),
        ("Connection not allowed", "Połączenie niedozwolone"),
        ("Legacy mode", "Tryb kompatybilności wstecznej (legacy)"),
        ("Map mode", "Tryb mapowania"),
        ("Translate mode", "Tryb translacji"),
        ("Use permanent password", "Użyj hasła permanentnego"),
        ("Use both passwords", "Użyj obu haseł"),
        ("Set permanent password", "Ustaw hasło permanentne"),
        ("Enable remote restart", "Włącz zdalne restartowanie"),
        ("Restart remote device", "Zrestartuj zdalne urządzenie"),
        ("Are you sure you want to restart", "Czy na pewno uruchomić ponownie"),
        ("Restarting remote device", "Trwa restartowanie Zdalnego Urządzenia"),
        ("remote_restarting_tip", "Trwa ponownie uruchomienie zdalnego urządzenia, zamknij ten komunikat i ponownie nawiąż za chwilę połączenie używając hasła permanentnego"),
        ("Copied", "Skopiowano"),
        ("Exit Fullscreen", "Wyłączyć tryb pełnoekranowy"),
        ("Fullscreen", "Tryb pełnoekranowy"),
        ("Mobile Actions", "Dostępne mobilne polecenia"),
        ("Select Monitor", "Wybierz ekran"),
        ("Control Actions", "Dostępne polecenia"),
        ("Display Settings", "Ustawienia wyświetlania"),
        ("Ratio", "Proporcje"),
        ("Image Quality", "Jakość obrazu"),
        ("Scroll Style", "Styl przewijania"),
        ("Show Toolbar", "Pokaż pasek narzędzi"),
        ("Hide Toolbar", "Ukryj pasek narzędzi"),
        ("Direct Connection", "Połączenie bezpośrednie"),
        ("Relay Connection", "Połączenie przez bramkę"),
        ("Secure Connection", "Połączenie szyfrowane"),
        ("Insecure Connection", "Połączenie nieszyfrowane"),
        ("Scale original", "Skaluj oryginalnie"),
        ("Scale adaptive", "Skaluj adaptacyjnie"),
        ("General", "Ogólne"),
        ("Security", "Zabezpieczenia"),
        ("Theme", "Motyw"),
        ("Dark Theme", "Ciemny motyw"),
        ("Light Theme", "Jasny motyw"),
        ("Dark", "Ciemny"),
        ("Light", "Jasny"),
        ("Follow System", "Zgodny z systemem"),
        ("Enable hardware codec", "Włącz akcelerację sprzętową kodeków"),
        ("Unlock Security Settings", "Odblokuj ustawienia zabezpieczeń"),
        ("Enable audio", "Włącz dźwięk"),
        ("Unlock Network Settings", "Odblokuj ustawienia Sieciowe"),
        ("Server", "Serwer"),
        ("Direct IP Access", "Bezpośredni adres IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Zastosuj"),
        ("Disconnect all devices?", "Czy rozłączyć wszystkie urządzenia?"),
        ("Clear", "Wyczyść"),
        ("Audio Input Device", "Urządzenie wejściowe Audio"),
        ("Use IP Whitelisting", "Użyj białej listy IP"),
        ("Network", "Sieć"),
        ("Pin Toolbar", "Przypnij pasek narzędzi"),
        ("Unpin Toolbar", "Odepnij pasek narzędzi"),
        ("Recording", "Nagrywanie"),
        ("Directory", "Folder"),
        ("Automatically record incoming sessions", "Automatycznie nagrywaj sesje przychodzące"),
        ("Change", "Zmień"),
        ("Start session recording", "Zacznij nagrywać sesję"),
        ("Stop session recording", "Zatrzymaj nagrywanie sesji"),
        ("Enable recording session", "Włącz nagrywanie sesji"),
        ("Enable LAN discovery", "Włącz wykrywanie urządzenia w sieci LAN"),
        ("Deny LAN discovery", "Zablokuj wykrywanie urządzenia w sieci LAN"),
        ("Write a message", "Napisz wiadomość"),
        ("Prompt", "Monit"),
        ("Please wait for confirmation of UAC...", "Poczekaj na potwierdzenie uprawnień UAC"),
        ("elevated_foreground_window_tip", "Aktualne okno zdalnego urządzenia wymaga wyższych uprawnień by poprawnie działać, chwilowo niemożliwym jest korzystanie z myszy i klawiatury. Możesz poprosić zdalnego użytkownika o minimalizację okna, lub nacisnąć przycisk podniesienia uprawnień w oknie zarządzania połączeniami. By uniknąć tego problemu, rekomendujemy instalację oprogramowania na urządzeniu zdalnym."),
        ("Disconnected", "Rozłączone"),
        ("Other", "Inne"),
        ("Confirm before closing multiple tabs", "Potwierdź przed zamknięciem wielu kart"),
        ("Keyboard Settings", "Ustawienia klawiatury"),
        ("Full Access", "Pełny dostęp"),
        ("Screen Share", "Udostępnianie ekranu"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland wymaga Ubuntu 21.04 lub nowszego."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland wymaga nowszej dystrybucji Linuksa. Wypróbuj pulpit X11 lub zmień system operacyjny."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Wybierz ekran do udostępnienia (działaj po zdalnego urządzenia)."),
        ("Show RustDesk", "Pokaż RustDesk"),
        ("This PC", "Ten komputer"),
        ("or", "lub"),
        ("Continue with", "Kontynuuj z"),
        ("Elevate", "Uzyskaj uprawnienia"),
        ("Zoom cursor", "Powiększenie kursora"),
        ("Accept sessions via password", "Uwierzytelnij sesję używając hasła"),
        ("Accept sessions via click", "Uwierzytelnij sesję poprzez kliknięcie"),
        ("Accept sessions via both", "Uwierzytelnij sesję za pomocą obu sposobów"),
        ("Please wait for the remote side to accept your session request...", "Oczekiwanie, na zatwierdzenie sesji przez host zdalny..."),
        ("One-time Password", "Hasło jednorazowe"),
        ("Use one-time password", "Użyj hasła jednorazowego"),
        ("One-time password length", "Długość hasła jednorazowego"),
        ("Request access to your device", "Żądanie dostępu do Twojego urządzenia"),
        ("Hide connection management window", "Ukryj okno zarządzania połączeniem"),
        ("hide_cm_tip", "Pozwalaj na ukrycie tylko, gdy akceptujesz sesje za pośrednictwem hasła i używasz hasła permanentnego"),
        ("wayland_experiment_tip", "Wsparcie dla Wayland jest niekompletne, użyj X11 jeżeli chcesz korzystać z dostępu nienadzorowanego"),
        ("Right click to select tabs", "Kliknij prawym przyciskiem myszy, aby wybrać zakładkę"),
        ("Skipped", "Pominięte"),
        ("Add to address book", "Dodaj do Książki Adresowej"),
        ("Group", "Grupy"),
        ("Search", "Szukaj"),
        ("Closed manually by web console", "Zakończone manualnie z konsoli Web"),
        ("Local keyboard type", "Lokalny typ klawiatury"),
        ("Select local keyboard type", "Wybierz lokalny typ klawiatury"),
        ("software_render_tip", "Jeżeli posiadasz kartę graficzną Nvidia i okno zamyka się natychmiast po nawiązaniu połączenia, instalacja sterownika nouveau i wybór renderowania programowego mogą pomóc. Restart aplikacji jest wymagany."),
        ("Always use software rendering", "Zawsze używaj renderowania programowego"),
        ("config_input", "By kontrolować zdalne urządzenie przy pomocy klawiatury, musisz udzielić aplikacji RustDesk uprawnień do \"Urządzeń Wejściowych\"."),
        ("config_microphone", "Aby umożliwić zdalne rozmowy należy przyznać RuskDesk uprawnienia do \"Nagrań audio\"."),
        ("request_elevation_tip", "Możesz poprosić o podniesienie uprawnień jeżeli ktoś posiada dostęp do zdalnego urządzenia."),
        ("Wait", "Czekaj"),
        ("Elevation Error", "Błąd przy podnoszeniu uprawnień"),
        ("Ask the remote user for authentication", "Poproś użytkownika zdalnego o uwierzytelnienie"),
        ("Choose this if the remote account is administrator", "Wybierz to jeżeli zdalne konto jest administratorem"),
        ("Transmit the username and password of administrator", "Prześlij nazwę użytkownika i hasło administratora"),
        ("still_click_uac_tip", "Nadal wymaga od zdalnego użytkownika potwierdzenia uprawnień UAC."),
        ("Request Elevation", "Poproś o podniesienie uprawnień"),
        ("wait_accept_uac_tip", "Prosimy czekać aż zdalny użytkownik potwierdzi uprawnienia UAC."),
        ("Elevate successfully", "Pomyślnie podniesiono uprawnienia"),
        ("uppercase", "wielkie litery"),
        ("lowercase", "małe litery"),
        ("digit", "cyfry"),
        ("special character", "znaki specjalne"),
        ("length>=8", "długość>=8"),
        ("Weak", "Słabe"),
        ("Medium", "Średnie"),
        ("Strong", "Mocne"),
        ("Switch Sides", "Zamień Strony"),
        ("Please confirm if you want to share your desktop?", "Czy na pewno chcesz udostępnić swój ekran?"),
        ("Display", "Wyświetlanie"),
        ("Default View Style", "Domyślny styl wyświetlania"),
        ("Default Scroll Style", "Domyślny styl przewijania"),
        ("Default Image Quality", "Domyślna jakość obrazu"),
        ("Default Codec", "Domyślny kodek"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Inne opcje domyślne"),
        ("Voice call", "Rozmowa głosowa"),
        ("Text chat", "Chat tekstowy"),
        ("Stop voice call", "Rozłącz"),
        ("relay_hint_tip", "Bezpośrednie połączenie może nie być możliwe, możesz spróbować połączyć się przez serwer przekazujący. \nDodatkowo, jeśli chcesz użyć serwera przekazującego przy pierwszej próbie, możesz dodać sufiks \"/r\" do identyfikatora lub wybrać opcję \"Zawsze łącz przez serwer przekazujący\" na karcie peer-ów."),
        ("Reconnect", "Połącz ponownie"),
        ("Codec", "Kodek"),
        ("Resolution", "Rozdzielczość"),
        ("No transfers in progress", "Brak transferów w toku"),
        ("Set one-time password length", "Ustaw długość jednorazowego hasła"),
        ("RDP Settings", "Ustawienia RDP"),
        ("Sort by", "Sortuj wg"),
        ("New Connection", "Nowe połączenie"),
        ("Restore", "Przywróć"),
        ("Minimize", "Minimalizuj"),
        ("Maximize", "Maksymalizuj"),
        ("Your Device", "Twoje urządzenie"),
        ("empty_recent_tip", "Ups, żadnych nowych sesji!\nCzas zaplanować nową."),
        ("empty_favorite_tip", "Brak ulubionych?\nZnajdźmy kogoś, z kim możesz się połączyć i dodaj Go do ulubionych!"),
        ("empty_lan_tip", "Ojej, wygląda na to, że nie odkryliśmy żadnych urządzeń z RustDesk w Twojej sieci."),
        ("empty_address_book_tip", "Ojej, wygląda na to, że nie ma żadnych wpisów w Twojej książce adresowej."),
        ("eg: admin", "np. admin"),
        ("Empty Username", "Pusty użytkownik"),
        ("Empty Password", "Puste hasło"),
        ("Me", "Ja"),
        ("identical_file_tip", "Ten plik jest identyczny z plikiem na drugim komputerze."),
        ("show_monitors_tip", "Pokaż monitory w zasobniku"),
        ("View Mode", "Tryb widoku"),
        ("login_linux_tip", "Musisz zalogować się na zdalne konto, by zezwolić na sesję pulpitu X"),
        ("verify_rustdesk_password_tip", "Weryfikuj hasło RustDesk"),
        ("remember_account_tip", "Zapamiętaj to konto"),
        ("os_account_desk_tip", "To konto jest używane do logowania do zdalnych systemów i włącza bezobsługowe sesje pulpitu"),
        ("OS Account", "Konto systemowe"),
        ("another_user_login_title_tip", "Inny użytkownik jest już zalogowany"),
        ("another_user_login_text_tip", "Rozłącz"),
        ("xorg_not_found_title_tip", "Nie znaleziono Xorg"),
        ("xorg_not_found_text_tip", "Proszę zainstalować Xorg"),
        ("no_desktop_title_tip", "Żaden pulpit nie jest dostępny"),
        ("no_desktop_text_tip", "Proszę zainstalować pulpit GNOME"),
        ("No need to elevate", "Podniesienie uprawnień nie jest wymagane"),
        ("System Sound", "Dźwięk systemowy"),
        ("Default", "Domyślne"),
        ("New RDP", "Nowe RDP"),
        ("Fingerprint", "Sygnatura"),
        ("Copy Fingerprint", "Skopiuj sygnaturę"),
        ("no fingerprints", "brak sygnatur"),
        ("Select a peer", "Wybierz zdalne urządzenie"),
        ("Select peers", "Wybierz zdalne urządzenia"),
        ("Plugins", "Wtyczki"),
        ("Uninstall", "Odinstaluj"),
        ("Update", "Aktualizuj"),
        ("Enable", "Włącz"),
        ("Disable", "Wyłącz"),
        ("Options", "Opcje"),
        ("resolution_original_tip", "Oryginalna rozdzielczość"),
        ("resolution_fit_local_tip", "Dostosuj rozdzielczość lokalną"),
        ("resolution_custom_tip", "Rozdzielczość niestandardowa"),
        ("Collapse toolbar", "Zwiń pasek narzędzi"),
        ("Accept and Elevate", "Akceptuj i Podnieś uprawnienia"),
        ("accept_and_elevate_btn_tooltip", "Zaakceptuj połączenie i podnieś uprawnienia UAC"),
        ("clipboard_wait_response_timeout_tip", "Upłynął limit czasu oczekiwania na schowek."),
        ("Incoming connection", "Połączenie przychodzące"),
        ("Outgoing connection", "Połączenie wychodzące"),
        ("Exit", "Wyjście"),
        ("Open", "Otwórz"),
        ("logout_tip", "Na pewno chcesz się wylogować?"),
        ("Service", "Usługa"),
        ("Start", "Uruchom"),
        ("Stop", "Zatrzymaj"),
        ("exceed_max_devices", "Przekroczona maks. liczba urządzeń"),
        ("Sync with recent sessions", "Synchronizacja z ostatnimi sesjami"),
        ("Sort tags", "Znaczniki sortowania"),
        ("Open connection in new tab", "Otwórz połączenie w nowej zakładce"),
        ("Move tab to new window", "Przenieś zakładkę do nowego okna"),
        ("Can not be empty", "Nie może być puste"),
        ("Already exists", "Już istnieje"),
        ("Change Password", "Zmień hasło"),
        ("Refresh Password", "Odśwież hasło"),
        ("ID", "ID"),
        ("Grid View", "Widok siatki"),
        ("List View", "Widok listy"),
        ("Select", "Wybierz"),
        ("Toggle Tags", "Przełącz tagi"),
        ("pull_ab_failed_tip", "Aktualizacja książki adresowej nie powiodła się"),
        ("push_ab_failed_tip", "Nie udało się zsynchronizować książki adresowej z serwerem"),
        ("synced_peer_readded_tip", "Urządzenia, które były obecne w ostatnich sesjach, zostaną ponownie dodane do książki adresowej"),
        ("Change Color", "Zmień kolor"),
        ("Primary Color", "Kolor podstawowy"),
        ("HSV Color", "Kolor HSV"),
        ("Installation Successful!", "Instalacja zakończona!"),
        ("Installation failed!", "Instalacja nie powiodła się"),
        ("Reverse mouse wheel", "Odwróć rolkę myszki"),
        ("{} sessions", "{} sesji"),
        ("scam_title", "Prawdopodobnie zostałeś OSZUKANY!"),
        ("scam_text1", "Jeżeli rozmawiasz przez telefon z osobą której NIE ZNASZ i NIE UFASZ, która prosi Cię o uruchomienie programu RustDesk i uruchomienia usługi - nie rób tego i natychmiast się rozłącz."),
        ("scam_text2", "Wygląda to na oszusta, który próbuje ukraść Twoje pieniądze lub inne prywatne informacje."),
        ("Don't show again", "Nie pokazuj więcej"),
        ("I Agree", "Zgadzam się"),
        ("Decline", "Odmawiam"),
        ("Timeout in minutes", "Czas bezczynności w minutach"),
        ("auto_disconnect_option_tip", "Automatycznie rozłącz sesje przychodzące przy braku aktywności użytkownika"),
        ("Connection failed due to inactivity", "Automatycznie rozłącz przy bezczynności"),
        ("Check for software update on startup", "Sprawdź aktualizacje przy starcie programu"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Proszę zaktualizować RustDesk Server Pro do wersji {} lub nowszej!"),
        ("pull_group_failed_tip", "Błąd odświeżania grup"),
        ("Filter by intersection", "Filtruj wg przecięcia"),
        ("Remove wallpaper during incoming sessions", "Usuń tapetę podczas sesji przychodzących"),
        ("Test", "Test"),
        ("display_is_plugged_out_msg", "Ekran został odłączony, przełącz się na pierwszy ekran."),
        ("No displays", "Brak ekranów"),
        ("elevated_switch_display_msg", "Przełącz się na ekran główny, ponieważ wyświetlanie kilku ekranów nie jest obsługiwane przy podniesionych uprawnieniach."),
        ("Open in new window", "Otwórz w nowym oknie"),
        ("Show displays as individual windows", "Pokaż ekrany w osobnych oknach"),
        ("Use all my displays for the remote session", "Użyj wszystkich moich ekranów do zdalnej sesji"),
        ("selinux_tip", "SELinux jest włączony na Twoim urządzeniu, co może przeszkodzić w uruchomieniu RustDesk po stronie kontrolowanej."),
        ("Change view", "Zmień widok"),
        ("Big tiles", "Duże kafelki"),
        ("Small tiles", "Małe kafelki"),
        ("List", "Lista"),
        ("Virtual display", "Witualne ekrany"),
        ("Plug out all", "Odłącz wszystko"),
        ("True color (4:4:4)", "True color (4:4:4)"),
        ("Enable blocking user input", "Zablokuj wprowadzanie danych przez użytkownika"),
        ("id_input_tip", "Możesz wprowadzić identyfikator, bezpośredni adres IP lub domenę z portem (<adres_domenowy>:<port>).\nJeżeli chcesz uzyskać dostęp do urządzenia na innym serwerze, dołącz adres serwera (<id>@<adres_serwera>?key=<wartość_klucza>, np. \n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nJeżeli chcesz uzyskać dostęp do urządzenia na serwerze publicznym, wpisz \"<id>@public\", klucz nie jest potrzebny dla serwera publicznego."),
        ("privacy_mode_impl_mag_tip", "Tryb 1"),
        ("privacy_mode_impl_virtual_display_tip", "Tryb 2"),
        ("Enter privacy mode", "Wejdź w tryb prywatności"),
        ("Exit privacy mode", "Wyjdź z trybu prywatności"),
        ("idd_not_support_under_win10_2004_tip", "Pośredni sterownik ekranu nie jest obsługiwany. Wymagany jest system Windows 10 w wersji 2004 lub nowszej."),
        ("switch_display_elevated_connections_tip", "Przełączanie na ekran inny niż główny nie jest obsługiwane przy podniesionych uprawnieniach, gdy istnieje wiele połączeń. Jeśli chcesz sterować wieloma ekranami, należy zainstalować program."),
        ("input_source_1_tip", "Wejście źródła 1"),
        ("input_source_2_tip", "Wejście źródła 2"),
        ("capture_display_elevated_connections_tip", "Przechwytywanie wielu ekranów nie jest obsługiwane w trybie użytkownika z podwyższonym poziomem uprawnień. Jeśli chcesz sterować wieloma wyświetlaczami, spróbuj ponownie po instalacji."),
        ("Swap control-command key", "Zamiana przycisków sterujących myszki"),
        ("swap-left-right-mouse", "Zamień przyciski myszki (lewy - prawy)"),
        ("2FA code", "Kod 2FA"),
        ("More", "Więcej"),
        ("enable-2fa-title", "Włącz autoryzację dwuskładnikową (2FA)"),
        ("enable-2fa-desc", "Skonfiguruj teraz swój moduł uwierzytelniający. Na telefonie lub komputerze możesz używać aplikacji autoryzującej, takiej jak Authy, Microsoft lub Google Authenticator.\n\nZeskanuj kod QR za pomocą aplikacji i wprowadź kod wyświetlany przez aplikację, aby włączyć uwierzytelnianie dwuskładnikowe."),
        ("wrong-2fa-code", "Nie można zweryfikować kodu. Sprawdź, czy kod oraz ustawienia lokalnego czasu są prawidłowe."),
        ("enter-2fa-title", "Autoryzacja dwuskładnikowa"),
        ("Email verification code must be 6 characters.", "Kod weryfikacyjny wysłany e-mailem musi mieć 6 znaków."),
        ("2FA code must be 6 digits.", "Kod 2FA musi zawierać 6 cyfr."),
        ("Multiple Windows sessions found", "Znaleziono wiele sesji Windows"),
        ("Please select the session you want to connect to", "Wybierz sesję, do której chcesz się podłączyć"),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
    ].iter().cloned().collect();
}
