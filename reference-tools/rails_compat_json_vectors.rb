# Generates vectors/rails_compat_json.json: how `ActiveSupport::JSON.encode` and `JSON.generate`
# write floats and strings, for crates/rails_compat/src/json.rs. Floats go through the JSON gem's
# own writer (Grisu2, not `Float#to_s`), so the cases include random doubles and short decimals,
# where Grisu2 sometimes picks a longer digit string than the shortest round trip.
#
#   reference-tools/run.sh reference-tools/rails_compat_json_vectors.rb
require_relative "support"

class RailsCompatJsonVectors
  EDGE_FLOATS = [
    0.0, -0.0, 1.0, -2.5, 320.0, 65.84, 29.97002997002997, 0.1 + 0.2, 9.64032,
    0.0001, 0.00001, 1.5e-7, 1e-6, 1e-7, 1.234e-9, 1e-10, 0.000123456789,
    1e14, 999999999999999.9, 1e15, 1234567890123456.0, 1e16, 12345678901234567.0, 2.0**60, 1e21, 1e22,
    1e100, 1e-100, 5e-324, 2.2250738585072014e-308, 1.7976931348623157e308
  ]

  def generate
    rng = Random.new(20260929)
    random = Array.new(1000) { [ rng.rand(2**64) ].pack("Q").unpack1("D") }.select(&:finite?)
    decimals = Array.new(1000) { "#{rng.rand(10**rng.rand(1..16))}e#{rng.rand(-12..24)}".to_f }
    floats = (EDGE_FLOATS + EDGE_FLOATS.map(&:-@) + random + decimals).uniq { |f| bits(f) }

    {
      "floats" => floats.map { |f| [ bits(f), ActiveSupport::JSON.encode(f) ] },
      "non_finite" => [ Float::INFINITY, -Float::INFINITY, Float::NAN ].map { |f| [ bits(f), ActiveSupport::JSON.encode(f) ] },
      "documents" => documents.map { |value| [ value, ActiveSupport::JSON.encode(value), JSON.generate(value) ] }
    }
  end

  private
    def bits(float)
      [ float ].pack("G").unpack1("H*")
    end

    def documents
      [
        "<a href=\"/x\">&</a>",
        "   \" \\ / \b\f\n\r\t \u0001\u001f\u007f é 😀",
        { "z" => 1, "a" => [ true, nil, 2.0, 1e-5, 0.1 + 0.2 ], "<k>" => { "&" => "<a & b>" } },
        [ 1, -1, 2**53, -2**63, 1e16, "x" ],
        "a\u2028b\u2029c",
        { "k\u2028" => "\u2029<>&" }
      ]
    end
end

path = File.join(ENV.fetch("VECTORS_DIR"), "rails_compat_json.json")
vectors = RailsCompatJsonVectors.new.generate
# One case per line: the float lists run to a couple of thousand rows.
body = vectors.map { |key, rows| "  #{JSON.generate(key)}: [\n#{rows.map { |row| "    #{JSON.generate(row)}" }.join(",\n")}\n  ]" }
File.write(path, "{\n#{body.join(",\n")}\n}\n")
puts "Wrote #{path}"
