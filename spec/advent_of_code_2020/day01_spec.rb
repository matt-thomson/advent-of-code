RSpec.describe AdventOfCode2020::Day01 do
  let(:fixture_path) { "spec/fixtures/day01.txt" }
  subject(:day01) { described_class.new(fixture_path) }

  describe "#part_one" do
    subject(:part_one) { day01.part_one }

    it { is_expected.to eq(514579) }
  end
end
