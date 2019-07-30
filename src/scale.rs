#![warn(clippy::all)]

use chrono::{NaiveDate, NaiveDateTime};
use decimal::d128;
use kairos::timetype::TimeType as TT;

pub enum Condomino {
    Michela,
    Gerardo,
    Elena,
    Giulia,
}

use Condomino as Co;

pub enum Operazione {
    VersamentoQuote(Condomino, d128),
    PagamentoScale,
    AltraSpesa(String, d128),
    AltroVersamento(String, d128),
    Prestito(d128),
    Restituzione(d128),
}
use Operazione as Op;

pub type Movimento = (NaiveDate, Operazione);

pub struct Param {
    costo_scale: d128,
    num_pulize_mese: u32,
    quota_mensile: d128,
}

pub type Attuale = (NaiveDate, Param);

pub macro_rules! from_ymd {
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
                costo_scale: d128!(20),
                num_pulize_mese: 2,
                quota_mensile: d128!(12),
            },
        );
        let condomini = [Co::Michela, Co::Gerardo, Co::Elena, Co::Giulia];
        let movimenti: Vec<Movimento> = vec![
            (
                tempo_zero,
                Op::AltroVersamento("Appianamento".to_string(), d128!(333)),
            ),
            (tempo_zero, Op::VersamentoQuote(Co::Michela, d128!(74))),
            (tempo_zero, Op::VersamentoQuote(Co::Gerardo, d128!(78))),
            (tempo_zero, Op::VersamentoQuote(Co::Elena, d128!(48))),
            (from_ymd!(2019, 7, 22), Op::Prestito(d128!(500))),
            (from_ymd!(2019, 7, 11), Op::PagamentoScale),
        ];
        Scale {
            tempo_zero,
            attuale,
            condomini,
            movimenti,
        }
    }

    fn contabile(&self, op: &Operazione) -> d128 {
        match *op {
            Op::VersamentoQuote(_, d) => d,
            Op::PagamentoScale => -self.attuale.1.costo_scale,
            Op::AltraSpesa(_, d) => -d,
            Op::AltroVersamento(_, d) => d,
            Op::Prestito(d) => -d,
            Op::Restituzione(d) => d,
        }
    }

    pub fn cassa(&self) -> d128 {
        let mut somma = d128!(0);
        for d in self.movimenti.iter().map(|(_, op)| self.contabile(op)) {
            somma += d
        }
        somma
    }

    fn altro_contabile(&self, op: &Operazione) -> d128 {
        match *op {
            Op::AltraSpesa(_, d) => -d,
            Op::AltroVersamento(_, d) => d,
            _ => d128!(0),
        }
    }

    pub fn tesoretto(&self, oggi: &NaiveDate) -> d128 {
        let mut altro = d128!(0);
        for d in self
            .movimenti
            .iter()
            .map(|(_, op)| self.altro_contabile(op))
        {
            altro += d
        }
        let mesi = since!(*oggi, self.tempo_zero) / 30;
        let num_condomini = self.condomini.len();

        let mut pagamenti = d128!(0);
        for d in self.movimenti.iter().map(|(_, op)| {
            if let Op::PagamentoScale = op {
                self.attuale.1.costo_scale
            } else {
                d128!(0)
            }
        }) {
            pagamenti += d
        }
        d128!(mesi * num_condomini) * self.attuale.1.quota_mensile - pagamenti;
    }
}
