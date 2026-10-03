### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = epoko { $epoch } de { $verifier }
    .keyword = demando

serve-answering-empty = iu petis la dosieron. ĉi tiu nodo havas nenion por doni.
    .keyword = malplena

serve-answering-gave = la dosieron al { $receiver } en epoko { $epoch }
    .keyword = donis

serve-answering-roll = { $members } el ni
    .keyword = nomlisto

serve-answering-cursed = { $name } demandis. 333 prenis { $milliseconds } milisekundojn de ties vivo,
    kiel ĝi faras ĉe ĉiu pordo.
    .keyword = malbeno

serve-answering-early = iu venis por esti demandata pri epoko { $asked_about }, kaj ĉi tiu nodo estas {
    ""}en { $now }
    .keyword = frua

serve-answering-witness = epoko { $epoch } respondita de { $prover }, kiu venis ĉi tien por esti
    demandata
    .keyword = atesto
