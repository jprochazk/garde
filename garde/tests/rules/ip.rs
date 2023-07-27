use super::util;
#[derive(Debug, garde::Validate)]
struct TestIpAny<'a> {
    #[garde(ip)]
    field: &'a str,
    #[garde(inner(ip))]
    inner: &'a [&'a str],
}

#[derive(Debug, garde::Validate)]
struct TestIpV4<'a> {
    #[garde(ipv4)]
    field: &'a str,
    #[garde(inner(ipv4))]
    inner: &'a [&'a str],
}

#[derive(Debug, garde::Validate)]
struct TestIpV6<'a> {
    #[garde(ipv6)]
    field: &'a str,
    #[garde(inner(ipv6))]
    inner: &'a [&'a str],
}

#[test]
fn ip_any_valid() {
    util::check_ok(
        &[
            TestIpAny {
                field: "1.1.1.1",
                inner: &["1.1.1.1"],
            },
            TestIpAny {
                field: "255.0.0.0",
                inner: &["255.0.0.0"],
            },
            TestIpAny {
                field: "0.0.0.0",
                inner: &["0.0.0.0"],
            },
            TestIpAny {
                field: "fe80::223:6cff:fe8a:2e8a",
                inner: &["fe80::223:6cff:fe8a:2e8a"],
            },
            TestIpAny {
                field: "::ffff:254.42.16.14",
                inner: &["::ffff:254.42.16.14"],
            },
        ],
        &(),
    )
}

#[test]
fn ip_any_invalid() {
    util::check_fail!(
        &[
            TestIpAny {
                field: "256.1.1.1",
                inner: &["256.1.1.1"]
            },
            TestIpAny {
                field: "25.1.1.",
                inner: &["25.1.1."]
            },
            TestIpAny {
                field: "25,1,1,1",
                inner: &["25,1,1,1"]
            },
            TestIpAny {
                field: "2a02::223:6cff :fe8a:2e8a",
                inner: &["2a02::223:6cff :fe8a:2e8a"]
            },
        ],
        &()
    )
}

#[test]
fn ip_v4_valid() {
    util::check_ok(
        &[
            TestIpV4 {
                field: "1.1.1.1",
                inner: &["1.1.1.1"],
            },
            TestIpV4 {
                field: "255.0.0.0",
                inner: &["255.0.0.0"],
            },
            TestIpV4 {
                field: "0.0.0.0",
                inner: &["0.0.0.0"],
            },
        ],
        &(),
    )
}

#[test]
fn ip_v4_invalid() {
    util::check_fail!(
        &[
            TestIpV4 {
                field: "256.1.1.1",
                inner: &["256.1.1.1"]
            },
            TestIpV4 {
                field: "25.1.1.",
                inner: &["25.1.1."]
            },
            TestIpV4 {
                field: "25,1,1,1",
                inner: &["25,1,1,1"]
            },
            TestIpV4 {
                field: "25.1 .1.1",
                inner: &["25.1 .1.1"]
            },
            TestIpV4 {
                field: "1.1.1.1\n",
                inner: &["1.1.1.1\n"]
            },
            TestIpV4 {
                field: "٧.2٥.3٣.243",
                inner: &["٧.2٥.3٣.243"]
            },
        ],
        &()
    )
}

#[test]
fn ip_v6_valid() {
    util::check_ok(
        &[
            TestIpV6 {
                field: "fe80::223:6cff:fe8a:2e8a",
                inner: &["fe80::223:6cff:fe8a:2e8a"],
            },
            TestIpV6 {
                field: "2a02::223:6cff:fe8a:2e8a",
                inner: &["2a02::223:6cff:fe8a:2e8a"],
            },
            TestIpV6 {
                field: "1::2:3:4:5:6:7",
                inner: &["1::2:3:4:5:6:7"],
            },
            TestIpV6 {
                field: "::",
                inner: &["::"],
            },
            TestIpV6 {
                field: "::a",
                inner: &["::a"],
            },
            TestIpV6 {
                field: "2::",
                inner: &["2::"],
            },
            TestIpV6 {
                field: "::ffff:254.42.16.14",
                inner: &["::ffff:254.42.16.14"],
            },
            TestIpV6 {
                field: "::ffff:0a0a:0a0a",
                inner: &["::ffff:0a0a:0a0a"],
            },
            TestIpV6 {
                field: "::254.42.16.14",
                inner: &["::254.42.16.14"],
            },
            TestIpV6 {
                field: "::0a0a:0a0a",
                inner: &["::0a0a:0a0a"],
            },
        ],
        &(),
    )
}

#[test]
fn ip_v6_invalid() {
    util::check_fail!(
        &[
            TestIpV6 {
                field: "foo",
                inner: &["foo"]
            },
            TestIpV6 {
                field: "127.0.0.1",
                inner: &["127.0.0.1"]
            },
            TestIpV6 {
                field: "12345::",
                inner: &["12345::"]
            },
            TestIpV6 {
                field: "1::2::3::4",
                inner: &["1::2::3::4"]
            },
            TestIpV6 {
                field: "1::zzz",
                inner: &["1::zzz"]
            },
            TestIpV6 {
                field: "1:2",
                inner: &["1:2"]
            },
            TestIpV6 {
                field: "fe80::223: 6cff:fe8a:2e8a",
                inner: &["fe80::223: 6cff:fe8a:2e8a"]
            },
            TestIpV6 {
                field: "2a02::223:6cff :fe8a:2e8a",
                inner: &["2a02::223:6cff :fe8a:2e8a"]
            },
            TestIpV6 {
                field: "::ffff:999.42.16.14",
                inner: &["::ffff:999.42.16.14"]
            },
            TestIpV6 {
                field: "::ffff:zzzz:0a0a",
                inner: &["::ffff:zzzz:0a0a"]
            },
        ],
        &()
    )
}
