# EXDGen

Auto-generate [EXDSchema](https://github.com/xivdev/EXDSchema)-based libraries, currently targeting Rust. This builds upon the existing Excel parsing in [Physis](https://github.com/redstrate/Physis) but with a higher-level, sheet-based structure for projects only care about the _data_.

You do not use this tool directly in your project, you can see it's output in [Physis Sheets](https://github.com/redstrate/PhysisSheets).

**NOTE:** This generator is still a WIP, the full schema is not supported yet.

# Usage

Run using `cargo run`, passing the path to the EXDSchema directory, your output directory and the applicable version:

```shell
cargo run -- "~/Downloads/EXDSchema" "~/sources/PhysisSheets" "2025.04.16.0000.0000"
```

For managing the sheets repository specifically, an even easier script is provided with `scripts/gen_repo.sh`:

```shell
./scripts/gen_repo.sh 2025.04.16.0000.0000 ../PhysisSheets
```

## License

![GPLv3](https://www.gnu.org/graphics/gplv3-127x51.png)

This project is licensed under the GNU General Public License 3.
