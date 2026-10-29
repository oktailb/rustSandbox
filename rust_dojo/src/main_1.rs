#[derive(Debug)]
struct Packet {
    id: u32,
    payload: String,
}

fn inspect_packet(p: &Packet) {
    println!("Inspecting packet #{}: {} bytes", p.id, p.payload.len());
}

fn main_1() {
    let mut pkt = Packet {
        id: 1,
        payload: String::from("INIT_HANDSHAKE"),
    };

    // Probleme 1 : je veux juste inspecter mon paquet, puis continuer a l'utiliser
    inspect_packet(&pkt);

    // Probleme 2 : je prends un pointeur/reference sur le payload pour le lire plus tard,
    // mais je modifie le paquet entre-temps.
    pkt.payload.push_str("_ACK");
    let view: &str = &pkt.payload;

    println!("Final view: {}", view);
}
