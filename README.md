# NEXGen

Auto-generate [EXDSchema](https://github.com/xivdev/EXDSchema)-based libraries, currently targetting Rust. This builds upon the existing Excel parsing in [Physis](https://github.com/redstrate/Physis) but with a higher-level, sheet-based structure for projects only care about the _data_.

You do not use NEXGen directly in your project, see it's output in [Physis Sheets](https://github.com/redstrate/PhysisSheets).

**NOTE:** This generator is still a WIP, the full schema is not supported yet.

# Usage

Run using `cargo run`, passing the path to the EXDSchema direcotry, and then your output directory:

```shell
cargo run -- "~/Downloads/EXDSchema" "~/sources/PhysisSheets"
```

## License

![GPLv3](https://www.gnu.org/graphics/gplv3-127x51.png)

This project is licensed under the GNU General Public License 3.
