#[cfg(test)]
mod tests {
    use crate::generate::GlyphGenerator;
    use crate::glyph::Glyph;
    use crate::glyph::InternalGlyph;
    use crate::parameters::Parameters;
    use crate::stroke::InternalStroke;
    use crate::stroke::Stroke;
    use std::fs;

    use relative_path::RelativePath;

    struct TestContext {
        basic: GlyphGenerator,
        advanced: GlyphGenerator,
    }

    fn load_parameters(parameter_file: &str) -> Parameters {
        let tests_dir = RelativePath::new("tests");
        let parameters_path = tests_dir.join(parameter_file).to_string();
        let parameters_json = fs::read_to_string(parameters_path)
            .expect("Failed to read parameters file")
            .to_string();
        let parameters: Parameters =
            serde_json::from_str(&parameters_json).expect("Error deserializing parameters JSON");
        return parameters;
    }

    fn setup() -> TestContext {
        return TestContext {
            basic: GlyphGenerator::new(load_parameters("parameters_4ap.json")),
            advanced: GlyphGenerator::new(load_parameters("parameters_9ap.json")),
        };
    }

    fn from_glyph(generator: &GlyphGenerator, glyph: &Glyph) -> InternalGlyph {
        glyph
            .strokes
            .iter()
            .map(|stroke| generator.from_stroke(stroke))
            .fold(InternalGlyph::empty(), |acc, stroke| acc.union(&stroke))
    }

    #[test]
    fn test_glyph_eq() {
        let a = InternalGlyph {
            strokes: vec![InternalStroke { index: 1 }, InternalStroke { index: 0 }],
            identifier: 3,
        };
        let b = InternalGlyph {
            strokes: vec![InternalStroke { index: 0 }, InternalStroke { index: 1 }],
            identifier: 3,
        };

        assert_eq!(a, b);
        assert!(a.eq(&b));
        assert!(b.eq(&a));

        let c = a.clone();
        let vect = vec![c];
        assert!(vect.contains(&a));
        assert!(vect.contains(&b));
    }

    #[test]
    fn test_generate_from_4_anchor_points_n_shape() {
        let strokes = vec![
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: -1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: 1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
        ];
        let context = setup();
        let generator = context.basic;
        let glyphs = generator.generate(&strokes, &strokes[0]);
        assert_eq!(3, glyphs.len());
    }

    #[test]
    fn test_generate_from_4_anchor_points_x_shape() {
        let strokes = vec![
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: -1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
            Stroke {
                x0: 1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
        ];
        let context = setup();
        let generator = context.basic;
        let glyphs = generator.generate(&strokes, &strokes[0]);
        assert_eq!(5, glyphs.len());
    }

    #[test]
    fn test_generate_from_4_anchor_points_box() {
        let strokes = vec![
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: -1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: 1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
        ];
        let context = setup();
        let generator = context.basic;
        let glyphs = generator.generate(&strokes, &strokes[0]);
        assert_eq!(15, glyphs.len());
    }

    #[test]
    fn test_generate_from_9_anchor_points_box() {
        let strokes = vec![
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: -1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: 1.0,
            },
            Stroke {
                x0: -1.0,
                y0: 1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: -1.0,
                y0: -1.0,
                x1: 1.0,
                y1: -1.0,
            },
            Stroke {
                x0: 1.0,
                y0: -1.0,
                x1: 1.0,
                y1: 1.0,
            },
        ];
        let context = setup();
        let generator = context.advanced;
        let glyphs = generator.generate(&strokes, &strokes[0]);
        assert_eq!(15, glyphs.len());
    }

    #[test]
    fn test_internal_single_stroke() {
        let context = setup();
        let generator = context.basic;

        for stroke in generator.parameters.parent_strokes.iter() {
            let glyph = generator.to_glyph(&generator.from_stroke(stroke));
            let stroke_ = &glyph.strokes[0];

            // Assert that the original stroke is equal to the first stroke of the generated glyph
            assert_eq!(
                stroke, stroke_,
                "Original stroke should match the stroke in the glyph"
            );

            // Convert the glyph back to an internal representation and then back to a glyph again
            let glyph_ = generator.to_glyph(&from_glyph(&generator, &glyph));

            // Assert that the two glyphs are equal
            assert_eq!(
                glyph, glyph_,
                "Glyph should be equal to itself after conversion round-trip"
            );
        }
    }

    #[test]
    fn test_internal_double_stroke() {
        let parameters_str = "{\"parent_strokes\":[{\"x0\":-1.0,\"y0\":-1.0,\"x1\":-1.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":1.0,\"y1\":-1.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":1.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":-1.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":1.0,\"y1\":-1.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":1.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":-1.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":1.0,\"y1\":1.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":-1.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":1.0,\"y0\":-1.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":1.0,\"x1\":-1.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":1.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":1.0,\"y0\":1.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":0.0,\"y0\":0.0,\"x1\":-1.0,\"y1\":0.0},{\"x0\":0.0,\"y0\":0.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":0.0,\"y0\":0.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":0.0,\"y0\":0.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":-1.0,\"y0\":0.0,\"x1\":1.0,\"y1\":0.0},{\"x0\":-1.0,\"y0\":0.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":-1.0,\"y0\":0.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":1.0,\"y0\":0.0,\"x1\":0.0,\"y1\":1.0},{\"x0\":1.0,\"y0\":0.0,\"x1\":0.0,\"y1\":-1.0},{\"x0\":0.0,\"y0\":1.0,\"x1\":0.0,\"y1\":-1.0}],\"intersection_matrix\":[[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,0,0,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,0,0,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1],[1,0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,0,0,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[1,0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,0],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1],[1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,1,1,1,1,1,0],[1,0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,0,1,1,0,1,1,1,1,1],[1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],[1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1],[1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1],[1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1],[1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,0,1],[1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1],[1,1,1,1,1,1,1,1,0,1,0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1],[1,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,0,1,1,1,1],[0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0,1,1,0,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1]],\"transformation_matrix\":[[0,15,9,1,9,15,1],[9,1,0,0,15,9,15],[8,8,8,2,2,2,8],[10,16,10,3,21,21,16],[11,18,13,7,24,23,20],[12,17,14,6,25,22,19],[14,19,12,5,22,25,17],[13,20,11,4,23,24,18],[2,2,2,8,8,8,2],[1,9,15,15,0,1,0],[3,21,21,16,10,16,3],[4,23,24,20,13,18,7],[5,22,25,19,14,17,6],[7,24,23,18,11,20,4],[6,25,22,17,12,19,5],[15,0,1,9,1,0,9],[21,3,3,10,16,10,21],[22,5,6,14,19,12,25],[23,4,7,13,20,11,24],[25,6,5,12,17,14,22],[24,7,4,11,18,13,23],[16,10,16,21,3,3,10],[17,12,19,25,6,5,14],[18,11,20,24,7,4,13],[20,13,18,23,4,7,11],[19,14,17,22,5,6,12],[26,27,28,29,28,27,29],[27,26,29,28,29,26,28],[29,28,27,27,26,29,26],[28,29,26,26,27,28,27],[30,30,35,35,35,30,35],[32,33,33,34,31,34,32],[31,34,31,32,33,33,34],[34,31,34,33,32,32,31],[33,32,32,31,34,31,33],[35,35,30,30,30,35,30]]}";
        let parameters: Parameters =
            serde_json::from_str(&parameters_str).expect("Error deserializing parameters JSON");
        let generator = GlyphGenerator::new(parameters);
        for s1 in generator.parameters.parent_strokes.iter() {
            let g1 = generator.from_stroke(s1);
            for s2 in generator.parameters.parent_strokes.iter() {
                let g2 = generator.from_stroke(s2);
                let g = {
                    let this = &g1;
                    let other = &g2;
                    let mut indices: Vec<usize> = this.strokes.iter().map(|s| s.index).collect();
                    indices.extend(other.strokes.iter().map(|s| s.index));

                    let unique_indices: Vec<usize> = indices
                        .into_iter()
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    let new_strokes: Vec<InternalStroke> = unique_indices
                        .into_iter()
                        .map(|index| InternalStroke { index })
                        .collect();

                    InternalGlyph {
                        strokes: new_strokes,
                        identifier: this.identifier | other.identifier,
                    }
                }; // Assuming a union method for combining

                if g1 == g2 {
                    assert_eq!(g, g1, "g should be equal to g1 if g1 equals g2");
                } else {
                    assert_eq!(g.strokes.len(), 2, "Glyph should have 2 strokes");
                }

                assert_eq!(g1.union(&g2), g2.union(&g1), "Union should be commutative");
            }
        }
    }

    #[test]
    fn test_intersection() {
        let context = setup();
        let generator = context.basic;

        // Convert parent strokes to InternalGlyphs
        let th = generator.from_stroke(&generator.parameters.parent_strokes[0]);
        let lv = generator.from_stroke(&generator.parameters.parent_strokes[1]);
        let d1 = generator.from_stroke(&generator.parameters.parent_strokes[2]);
        let d2 = generator.from_stroke(&generator.parameters.parent_strokes[3]);
        let rv = generator.from_stroke(&generator.parameters.parent_strokes[4]);
        let bh = generator.from_stroke(&generator.parameters.parent_strokes[5]);

        // Perform intersection checks
        assert!(
            !generator.are_strokes_intersecting(&th.union(&bh)),
            "th and bh should not intersect"
        );
        assert!(
            !generator.are_strokes_intersecting(&lv.union(&rv)),
            "lv and rv should not intersect"
        );
        assert!(
            generator.are_strokes_intersecting(&d1.union(&d2)),
            "d1 and d2 should intersect"
        );
        assert!(
            generator.are_strokes_intersecting(&lv.union(&d1)),
            "lv and d1 should intersect"
        );
        assert!(
            generator.are_strokes_intersecting(&bh.union(&d2)),
            "bh and d2 should intersect"
        );
        assert!(
            generator.are_strokes_intersecting(&rv.union(&th)),
            "rv and th should intersect"
        );
        assert!(
            generator.are_strokes_intersecting(
                &th.union(&lv).union(&d1).union(&d2).union(&rv).union(&bh)
            ),
            "All combined should intersect"
        );
    }

    #[test]
    fn test_flip_horizontal() {
        let context = setup();
        let generator = context.basic;

        // Assuming from_stroke is a method that transforms a stroke reference into an InternalGlyph
        let strokes = generator
            .parameters
            .parent_strokes
            .iter()
            .map(|stroke| generator.from_stroke(stroke))
            .collect::<Vec<_>>();
        let (th, lv, d1, d2, rv, bh) = (
            &strokes[0],
            &strokes[1],
            &strokes[2],
            &strokes[3],
            &strokes[4],
            &strokes[5],
        );

        for stroke in [th, lv, rv, bh] {
            assert!(
                generator.transform(&th).iter().any(|s| s == stroke),
                "Stroke should be in the transformed set"
            );
        }

        for stroke in [d1, d2] {
            assert!(
                generator.transform(&d1).iter().any(|s| s == stroke),
                "Stroke should be in the transformed set"
            );
        }

        let transformed = generator.transform(&lv.union(&bh).union(&d1));
        assert!(
            transformed.iter().any(|g| *g == rv.union(&th).union(&d1)),
            "Combined transformation should be in the set"
        );
    }
}
