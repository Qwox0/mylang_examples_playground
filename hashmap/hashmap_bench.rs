type Key = i32;

fn main() {
    /*
    let mut keys = Vec::<Key>::with_capacity(N);
    for i in 0..N {
        keys.push(i as Key);
    }

    /*

    mut map_alloc_buf := u8.[0; 5 * 1024 * 1024];
    mut map_alloc := std.mem.FixedArenaAllocator.new(map_alloc_buf[..]mut);
    map_alloc := map_alloc.&mut.allocator();
    //map_alloc := std.ctx.alloc;
    */


    let mut map = std::collections::HashMap::<i32, i32>::new();

    for &k in keys.as_slice() {
        map.insert(k, 12345);
    }

    let elapsed = now.elapsed();

    println!("size = {}", map.len());

    println!("{} s; {} ns", elapsed.as_secs(), elapsed.subsec_nanos());
    println!("{}", elapsed.as_secs_f32());
    */

    let mut rng = Sfc64::from_seed(213);

    let now = std::time::Instant::now();

    let mut map = std::collections::HashMap::<i32, i32>::new();

    const N: usize = 100_000_000;

    for n in 0..N {
        map.insert(rng.next() as i32, 0);
    }

    let elapsed = now.elapsed();

    let result = map.len();
    assert_eq!(result, 98841586);

    println!("rust; ?; InsertHugeInt; ?; insert 100M int; {result}; {}; ?", elapsed.as_secs_f32());
}

#[derive(Debug)]
struct Sfc64 {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl Sfc64 {
    fn default() -> Sfc64 {
        Self::from_seed(0x853c49e6748fea9b)
    }

    fn from_seed(seed: u64) -> Sfc64 {
        let mut slf = Sfc64 { a: seed, b: seed, c: seed, counter: 1 };
        for i in 0..12 {
            slf.next();
        }
        slf
    }

    fn next(&mut self) -> u64 {
        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter += 1;
        self.a = self.b ^ (self.b >> 11);
        self.b = self.c.wrapping_add(self.c << 3);
        self.c = ((self.c << 24) | (self.c >> 40)).wrapping_add(tmp);
        return tmp;
    }
}

/*// this is probably the fastest high quality 64bit random number generator that exists.
// Implements Small Fast Counting v4 RNG from PractRand.
class sfc64 {
public:
    using result_type = uint64_t;

    // no copy ctors so we don't accidentally get the same random again
    sfc64(sfc64 const&) = delete;
    sfc64& operator=(sfc64 const&) = delete;

    sfc64(sfc64&&) = default;
    sfc64& operator=(sfc64&&) = default;

    sfc64(std::array<uint64_t, 4> const& state)
        : m_a(state[0])
        , m_b(state[1])
        , m_c(state[2])
        , m_counter(state[3]) {}

    static constexpr uint64_t(min)() {
        return (std::numeric_limits<uint64_t>::min)();
    }
    static constexpr uint64_t(max)() {
        return (std::numeric_limits<uint64_t>::max)();
    }

    sfc64()
        : sfc64(UINT64_C(0x853c49e6748fea9b)) {}

    sfc64(uint64_t seed)
        : m_a(seed)
        , m_b(seed)
        , m_c(seed)
        , m_counter(1) {
        for (int i = 0; i < 12; ++i) {
            operator()();
        }
    }

    void seed() {
        *this = sfc64{std::random_device{}()};
    }

    uint64_t operator()() noexcept {
        auto const tmp = m_a + m_b + m_counter++;
        m_a = m_b ^ (m_b >> right_shift);
        m_b = m_c + (m_c << left_shift);
        m_c = rotl(m_c, rotation) + tmp;
        return tmp;
    }

    // this is a bit biased, but for our use case that's not important.
    uint64_t operator()(uint64_t boundExcluded) noexcept {
#ifdef __SIZEOF_INT128__
        return static_cast<uint64_t>((static_cast<unsigned __int128>(operator()()) * static_cast<unsigned __int128>(boundExcluded)) >> 64u);
#endif
    }

    std::array<uint64_t, 4> state() const {
        return {m_a, m_b, m_c, m_counter};
    }

    void state(std::array<uint64_t, 4> const& s) {
        m_a = s[0];
        m_b = s[1];
        m_c = s[2];
        m_counter = s[3];
    }

private:
    template <typename T>
    T rotl(T const x, int k) {
        return (x << k) | (x >> (8 * sizeof(T) - k));
    }

    static constexpr int rotation = 24;
    static constexpr int right_shift = 11;
    static constexpr int left_shift = 3;
    uint64_t m_a;
    uint64_t m_b;
    uint64_t m_c;
    uint64_t m_counter;
};
*/
