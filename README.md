# cffdrs-rs

[![docs.rs](https://img.shields.io/docsrs/cffdrs)](https://docs.rs/cffdrs)
[![Crates.io Version](https://img.shields.io/crates/v/cffdrs)](https://crates.io/crates/cffdrs)

Rust implementation of the Canadian Forest Fire Danger Rating System

> Note: This crate is a work-in-progress and likely doesn't correctly implement all equations.

## Instructions

### Run tests

Tests are run using [nextest](https://github.com/nextest-rs/nextest). You may need to install it first.

```bash
cargo install cargo-nextest --locked # Only if you haven't installed it yet
cargo nextest run
```

### Workspace tasks

`cargo-workspace` is used to make managing the multiple crates in this repo easier.

For example:

```bash
cargo install cargo-workspace --locked

cargo ws publish # Publish new version of crates
cargo ws create crates/my-new-crate # Create a new crate
```

## Releases

Releases are managed via [`release-please`](https://github.com/googleapis/release-please). Most of the time, it should just automatically work.
The only thing that may be required is to update `release-please-config.json` with any packages in order for `release-please` to keep track of it.

> Note: it's required to use [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) in order for `release-please` to properly
> keep track of commits.

## References

Information Report GLC-X-10
Wotton, B.M.; Alexander, M.E.; Taylor, S.W. 2009. Updates and
Revisions to the 1992 Canadian Forest Fire Behavior Prediction
System. Natural Resources Canada, Canadian Forest Service,
Great Lakes Forestry Centre, Sault Ste. Marie, Ontario, Canada.
Information Report GLC-X-10, 45p.
[Link](https://publications.gc.ca/site/eng/9.505580/publication.html)
[View PDF](https://cfs.nrcan.gc.ca/pubwarehouse/pdfs/31414.pdf)

Forestry Canada. 1992. Development and structure of the Canadian Forest Fire Behavior Prediction System. Forestry Canada, Headquarters, Fire Danger Group and Science and Sustainable Development Directorate, Ottawa. Information Report ST-X-3. 64 p.
[Link](https://ostrnrcan-dostrncan.canada.ca/entities/publication/27d3ea09-fd84-4653-a22e-598cc597400c)

Wang, X.; Wotton, M.; Cantin, A.; Parisien, M.; Anderson, K.; Moore, B.; Flannigan, M. 2017. cffdrs: An R package for the Canadian Forest Fire Danger Rating System. Ecological Processes 6(5): 11p.
[https://github.com/cffdrs/cffdrs_r](https://github.com/cffdrs/cffdrs_r)

Van Wagner, C.E.; Pickett, T.L. 1985. Equations and FORTRAN program for the Canadian Forest Fire Weather Index System. Canadian Forestry Service, Petawawa National Forestry Institute, Chalk River, Ontario. Forestry Technical Report 33. 18 p.
[View PDF](https://cfs.nrcan.gc.ca/pubwarehouse/pdfs/19973.pdf)
