#![warn(clippy::all)]

extern crate chrono;
use chrono::NaiveDate;

pub enum Condomino {
    Michela,
    Gerardo,
    Elena,
    Giulia,
}

use Condomino as Co;

pub enum Operazione {
    VersamentoQuote(Condomino, u32),
    PagamentoScale,
    AltraSpesa(String, u32),
    AltroVersamento(String, u32),
    Prestito(u32),
    Restituzione(u32),
}
use Operazione as Op;

pub type Movimento = (NaiveDate, Operazione);

pub struct Param {
    costo_scale: u32,
    num_pulize_mese: u32,
    quota_mensile: u32,
}

pub type Attuale = (NaiveDate, Param);

#[macro_export]
macro_rules! from_ymd {
    ($y:expr, $m:expr, $d:expr) => {
        <NaiveDate>::from_ymd($y, $m, $d);
    };
}

macro_rules! since {
    ($d1:expr, $d2:expr) => {
        <NaiveDate>::signed_duration_since($d1, $d2);
    };
}

const ANNO_ZERO: i32 = 2019;
const MESE_ZERO: u32 = 7;
const GIORNO_ZERO: u32 = 1;

pub struct Scale {
    tempo_zero: NaiveDate,
    attuale: Attuale,
    condomini: [Condomino; 4],
    movimenti: Vec<Movimento>,
}

impl Scale {
    pub fn new() -> Scale {
        let tempo_zero = from_ymd!(ANNO_ZERO, MESE_ZERO, GIORNO_ZERO);
        let attuale: Attuale = (
            tempo_zero,
            Param {
                costo_scale: 20,
                num_pulize_mese: 2,
                quota_mensile: 12,
            },
        );
        let condomini = [Co::Michela, Co::Gerardo, Co::Elena, Co::Giulia];
        let movimenti: Vec<Movimento> = vec![
            (
                tempo_zero,
                Op::AltroVersamento("Appianamento".to_string(), 333),
            ),
            (tempo_zero, Op::VersamentoQuote(Co::Michela, 74)),
            (tempo_zero, Op::VersamentoQuote(Co::Gerardo, 78)),
            (tempo_zero, Op::VersamentoQuote(Co::Elena, 48)),
            (from_ymd!(2019, 7, 22), Op::Prestito(500)),
            (from_ymd!(2019, 7, 11), Op::PagamentoScale),
        ];
        Scale {
            tempo_zero,
            attuale,
            condomini,
            movimenti,
        }
    }

    fn contabile(&self, op: &Operazione) -> i32 {
        match *op {
            Op::VersamentoQuote(_, u) => u as i32,
            Op::PagamentoScale => -(self.attuale.1.costo_scale as i32),
            Op::AltraSpesa(_, u) => -(u as i32),
            Op::AltroVersamento(_, u) => u as i32,
            Op::Prestito(u) => -(u as i32),
            Op::Restituzione(u) => u as i32,
        }
    }

    pub fn cassa(&self) -> i32 {
        let mut somma = 0;
        for i in self.movimenti.iter().map(|(_, op)| self.contabile(op)) {
            somma += i
        }
        somma
    }

    fn altro_contabile(&self, op: &Operazione) -> i32 {
        match *op {
            Op::AltraSpesa(_, u) => -(u as i32),
            Op::AltroVersamento(_, u) => u as i32,
            _ => 0,
        }
    }

    pub fn tesoretto(&self, oggi: NaiveDate) -> i32 {
        let mut altro = 0;
        for i in self
            .movimenti
            .iter()
            .map(|(_, op)| self.altro_contabile(op))
        {
            altro += i
        }

        let mut pagamenti = 0;
        for i in self.movimenti.iter().map(|(_, op)| {
            if let Op::PagamentoScale = op {
                self.attuale.1.costo_scale as i32
            } else {
                0
            }
        }) {
            pagamenti += i
        }

        let mesi = since!(oggi, self.tempo_zero).num_days() as i32 / 30;
        let num_condomini = self.condomini.len() as i32;
        mesi * num_condomini * self.attuale.1.quota_mensile as i32 + altro - pagamenti
    }
}
