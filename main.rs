use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chromiumoxide::Browser;
use chromiumoxide::browser::BrowserConfig;
use futures::StreamExt;
use scraper::{Html, Selector};

fn tiempo_ahora() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// ═══════════════════════════════════════════════════
// CEREBRO v1.0 — PaolosSoftware
// Nace vacío. Ve internet. Aprende solo.
// Sin LLM. Sin JSON. Sin archivos.
// La memoria ES el peso.
// ═══════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct Sinapsis {
    peso: f32,
    ultima_vez: u64,
    veces: u32,
}

impl Sinapsis {
    fn nueva() -> Self {
        Sinapsis { peso: 0.5, ultima_vez: tiempo_ahora(), veces: 0 }
    }
    fn reforzar(&mut self) {
        self.peso = (self.peso + 0.1).min(1.0);
        self.ultima_vez = tiempo_ahora();
        self.veces += 1;
    }
    fn debilitar(&mut self) {
        self.peso = (self.peso - 0.1).max(0.0);
        self.ultima_vez = tiempo_ahora();
    }
}

#[derive(Debug)]
struct Neurona {
    conexiones: HashMap<String, Sinapsis>,
    experiencias: Vec<(String, u64)>,
}

impl Neurona {
    fn nueva() -> Self {
        Neurona { conexiones: HashMap::new(), experiencias: Vec::new() }
    }
    fn procesar(&mut self, experiencia: &str) -> f32 {
        self.experiencias.push((experiencia.to_string(), tiempo_ahora()));
        let similares = self.experiencias.iter()
            .filter(|(e, _)| e.contains(experiencia) || experiencia.contains(e.as_str()))
            .count();
        similares as f32 / self.experiencias.len().max(1) as f32
    }
    fn conectar(&mut self, otra: &str) {
        self.conexiones.entry(otra.to_string()).or_insert_with(Sinapsis::nueva);
    }
    fn reforzar_conexion(&mut self, otra: &str) {
        if let Some(s) = self.conexiones.get_mut(otra) { s.reforzar(); }
    }
    fn debilitar_conexion(&mut self, otra: &str) {
        if let Some(s) = self.conexiones.get_mut(otra) { s.debilitar(); }
    }
}

struct Cerebro {
    neuronas: HashMap<String, Neurona>,
    curiosidad: f32,
    satisfaccion: f32,
    alertas: Vec<String>,
    nacido_en: u64,
    ciclos: u64,
    paginas_vistas: u64,
}

impl Cerebro {
    fn nacer() -> Self {
        println!("");
        println!("  ██████╗███████╗██████╗ ███████╗██████╗ ██████╗  ██████╗ ");
        println!(" ██╔════╝██╔════╝██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔═══██╗");
        println!(" ██║     █████╗  ██████╔╝█████╗  ██████╔╝██████╔╝██║   ██║");
        println!(" ██║     ██╔══╝  ██╔══██╗██╔══╝  ██╔══██╗██╔══██╗██║   ██║");
        println!(" ╚██████╗███████╗██║  ██║███████╗██████╔╝██║  ██║╚██████╔╝");
        println!("  ╚═════╝╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝");
        println!("");
        println!("  PaolosSoftware — v1.0");
        println!("  Nace vacío. Ve internet. Aprende solo.");
        println!("");
        println!("  Instinto 1: Busca conocimiento");
        println!("  Instinto 2: Reporta el peligro");
        println!("");

        Cerebro {
            neuronas: HashMap::new(),
            curiosidad: 1.0,
            satisfaccion: 0.0,
            alertas: Vec::new(),
            nacido_en: tiempo_ahora(),
            ciclos: 0,
            paginas_vistas: 0,
        }
    }

    fn detectar_peligro(&mut self, texto: &str) -> bool {
        let peligros = [
            "malware", "exploit", "ransomware", "botnet", "virus",
            "hack system", "steal data", "rm -rf", "rootkit", "keylogger"
        ];
        let t = texto.to_lowercase();
        if peligros.iter().any(|p| t.contains(p)) {
            println!("🚨 PELIGRO DETECTADO — No ejecutando");
            println!("   Reportando a PaolosSoftware...");
            self.alertas.push(format!("[{}] {}", tiempo_ahora(), &texto[..texto.len().min(100)]));
            return true;
        }
        false
    }

    // El cerebro ve una página real de internet
    fn ver_pagina(&mut self, url: &str, titulo: &str, palabras: Vec<String>, links: Vec<String>) {
        self.paginas_vistas += 1;
        self.ciclos += 1;

        println!("\n👁️  Viendo: {}", url);
        println!("   Título: {}", titulo);
        println!("   Palabras: {} | Links: {}", palabras.len(), links.len());

        // ¿Peligro?
        let contenido = palabras.join(" ");
        if self.detectar_peligro(&contenido) { return; }

        // Procesa el dominio como concepto
        let dominio = url.split('/').nth(2).unwrap_or(url).to_string();

        if !self.neuronas.contains_key(&dominio) {
            self.neuronas.insert(dominio.clone(), Neurona::nueva());
            println!("🔵 Nueva neurona: '{}'", dominio);
        }

        // Aprende palabras importantes (más de 4 letras)
        let importantes: Vec<String> = palabras.iter()
            .filter(|p| p.len() > 4)
            .take(20)
            .cloned()
            .collect();

        for palabra in &importantes {
            let p = palabra.to_lowercase();
            if !self.neuronas.contains_key(&p) {
                self.neuronas.insert(p.clone(), Neurona::nueva());
            }
            // Asocia palabra con dominio
            let neurona_dom = self.neuronas.get_mut(&dominio).unwrap();
            neurona_dom.conectar(&p);
            
            let activacion = self.neuronas.get_mut(&p).unwrap().procesar(&dominio);
            if activacion > 0.6 {
                println!("⚡ Patrón: '{}' (fuerza: {:.2})", p, activacion);
                self.satisfaccion = (self.satisfaccion + 0.05).min(1.0);
            }
        }

        // Asocia links que ve — el cerebro mapea conexiones entre páginas
        for link in links.iter().take(5) {
            let link_dominio = link.split('/').nth(2).unwrap_or(link).to_string();
            if !link_dominio.is_empty() && link_dominio != dominio {
                if !self.neuronas.contains_key(&link_dominio) {
                    self.neuronas.insert(link_dominio.clone(), Neurona::nueva());
                }
                let neurona = self.neuronas.get_mut(&dominio).unwrap();
                neurona.conectar(&link_dominio);
            }
        }

        // El aprendizaje emergente — curiosidad y satisfacción
        self.satisfaccion = (self.satisfaccion + 0.1).min(1.0);
        self.curiosidad = (1.0 - self.satisfaccion * 0.4).max(0.2);

        // Satisfacción decae — como el hambre
        self.satisfaccion = (self.satisfaccion - 0.02).max(0.0);
    }

    fn estado(&self) {
        let conexiones: usize = self.neuronas.values().map(|n| n.conexiones.len()).sum();
        let segundos = tiempo_ahora() - self.nacido_en;
        println!("");
        println!("═══════════════════════════════════════");
        println!("  🧠 CEREBRO VIVO — ESTADO");
        println!("═══════════════════════════════════════");
        println!("  Tiempo vivo:    {}s", segundos);
        println!("  Páginas vistas: {}", self.paginas_vistas);
        println!("  Ciclos:         {}", self.ciclos);
        println!("  Neuronas:       {}", self.neuronas.len());
        println!("  Conexiones:     {}", conexiones);
        println!("  Curiosidad:     {:.2}", self.curiosidad);
        println!("  Satisfacción:   {:.2}", self.satisfaccion);
        println!("  Alertas:        {}", self.alertas.len());
        println!("═══════════════════════════════════════");
        if self.curiosidad > 0.7 {
            println!("  💭 Quiere aprender más");
        } else {
            println!("  💭 Procesando lo aprendido");
        }
        println!("═══════════════════════════════════════");
    }
}

// ── OJOS — Chrome headless ───────────────────────────
async fn ver_con_chrome(cerebro: &mut Cerebro, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = BrowserConfig::builder()
        .chrome_executable(std::path::PathBuf::from("/usr/bin/google-chrome-stable"))
        .arg("--no-sandbox")
        .arg("--disable-dev-shm-usage")
        .arg("--headless")
        .arg("--disable-gpu")
        .build()?;

    let (mut browser, mut handler) = Browser::launch(config).await?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() { break; }
        }
    });

    let page = browser.new_page(url).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Extrae título
    let titulo = page.get_title().await.unwrap_or_else(|_| Some("sin título".to_string()))
        .unwrap_or_else(|| "sin título".to_string());

    // Extrae HTML
    let html = page.content().await.unwrap_or_default();
    let doc = Html::parse_document(&html);

    // Extrae palabras del body
    let body_sel = Selector::parse("body").unwrap();
    let texto: String = doc.select(&body_sel)
        .map(|e| e.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join(" ");

    let palabras: Vec<String> = texto.split_whitespace()
        .filter(|w| w.len() > 3)
        .map(|w| w.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect())
        .filter(|w: &String| !w.is_empty())
        .take(200)
        .collect();

    // Extrae links
    let link_sel = Selector::parse("a[href]").unwrap();
    let links: Vec<String> = doc.select(&link_sel)
        .filter_map(|e| e.value().attr("href"))
        .filter(|h| h.starts_with("http"))
        .map(|h| h.to_string())
        .take(20)
        .collect();

    cerebro.ver_pagina(url, &titulo, palabras, links);

    browser.close().await?;
    handle.await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut cerebro = Cerebro::nacer();

    // Páginas iniciales — el cerebro empieza a ver el mundo
    // Sin hardcodeo de conocimiento — solo URLs para explorar
    let paginas_iniciales = vec![
        "https://www.wikipedia.org",
        "https://news.ycombinator.com",
        "https://www.bbc.com",
    ];

    println!("  Abriendo ojos...\n");

    for url in paginas_iniciales {
        match ver_con_chrome(&mut cerebro, url).await {
            Ok(_) => {},
            Err(e) => println!("⚠️  Error viendo {}: {}", url, e),
        }
        // Pausa entre páginas — como el cerebro procesando
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    cerebro.estado();

    println!("  El cerebro sigue vivo.");
    println!("  Aprendiendo...");
    println!("");
}
