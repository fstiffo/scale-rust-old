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

use Condomino as Cond;

pub enum Operazione {
    VersamentoQuote(Condomino, d128),
    PagamentoScale,
    AltraSpesa(String, d128),
    AltroVersamento(String, d128),
    Prestito(d128),
    Restituzione(d128),
}
use Operazione as Op;

pub type Movimento = (NaiveDateTime, Operazione);

pub struct Param {
    costo_scale: d128,
    num_pulize_mese: u32,
    quota_mensile: d128,
}

pub type Attuale = (NaiveDateTime, Param);

macro_rules! naive_date {
    ($y:expr, $m:expr, $d:expr) => {
        <NaiveDate>::from_ymd($y, $m, $d).and_hms(0, 0, 0);
    };
}

const ANNO_ZERO: i32 = 2019;
const MESE_ZERO: u32 = 7;
const GIORNO_ZERO: u32 = 1;

pub struct Scale {
    tempo_zero: NaiveDateTime,
    attuale: Attuale,
    condomini: [Condomino; 4],
    movimenti: Vec<Movimento>,
}

impl Scale {
    pub fn new() -> Scale {
        let tempo_zero = naive_date!(ANNO_ZERO, MESE_ZERO, GIORNO_ZERO);
        let attuale: Attuale = (
            tempo_zero,
            Param {
                costo_scale: d128!(20),
                num_pulize_mese: 2,
                quota_mensile: d128!(12),
            },
        );
        let condomini = [Cond::Michela, Cond::Gerardo, Cond::Elena, Cond::Giulia];
        let movimenti: Vec<Movimento> = vec![
            (
                tempo_zero,
                Op::AltroVersamento("Appianamento".to_string(), d128!(333)),
            ),
            (tempo_zero, Op::VersamentoQuote(C::Michela, d128!(74))),
            (tempo_zero, Op::VersamentoQuote(C::Gerardo, d128!(78))),
            (tempo_zero, Op::VersamentoQuote(C::Elena, d128!(48))),
            (naive_date!(2019, 7, 22), Op::Prestito(d128!(500))),
            (naive_date!(2019, 7, 11), Op::PagamentoScale),
        ];
        Scale {
            tempo_zero,
            attuale,
            condomini,
            movimenti,
        }
    }
}
