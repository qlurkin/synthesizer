use fundsp::hacker::*;

pub struct Track {
    pub chains: Vec<Option<usize>>,
    pub event_id: Option<EventId>,
    pub mix_level: Shared,
    pub snoop_l: Snoop,
    pub snoop_r: Snoop,
    pub sequencer: Sequencer,
    pub net: Net,
    pub chain_cursor: usize,
    pub phrase_cursor: usize,
    pub step_cursor: usize,
}

impl Track {
    pub fn new() -> Self {
        let (snoop_l, snoop_l_backend) = snoop(2048);
        let (snoop_r, snoop_r_backend) = snoop(2048);

        let mut sequencer = Sequencer::new(false, 8);

        let backend = sequencer.backend();

        let mix_level = shared(1.0);

        let mut net = Net::wrap(Box::new(backend));
        net = net
            >> (((multipass::<U2>() * (var(&mix_level) | var(&mix_level)))
                >> (snoop_l_backend | snoop_r_backend))
                | multipass::<U6>());

        Self {
            chains: std::iter::repeat_with(|| None).take(256).collect(),
            event_id: None,
            mix_level,
            snoop_l,
            snoop_r,
            sequencer,
            net,
            chain_cursor: 0,
            phrase_cursor: 0,
            step_cursor: 0,
        }
    }

    pub fn step(&mut self) {
        self.step_cursor = (self.step_cursor + 1) % 16;
    }
}
