# frozen_string_literal: true

require 'advent_of_code_2020/day15/sequence'

RSpec.describe AdventOfCode2020::Day15::Sequence do
  subject(:sequence) { described_class.new(starting_numbers) }

  describe '#numbers' do
    subject(:numbers) { sequence.numbers.take(10).to_a }

    let(:starting_numbers) { [0, 3, 6] }

    it { is_expected.to eq([0, 3, 6, 0, 3, 3, 1, 0, 4, 0]) }
  end
end
