# frozen_string_literal: true

require 'advent_of_code2017/day02'

RSpec.describe AdventOfCode2017::Day02 do
  describe '.solve_part_one' do
    subject { described_class.solve_part_one(input) }

    let(:input) do
      <<~INPUT
        5 1 9 5
        7 5 3
        2 4 6 8
      INPUT
    end

    it { is_expected.to eq(18) }
  end

  describe '.solve_part_two' do
    subject { described_class.solve_part_two(input) }

    let(:input) do
      <<~INPUT
        5 9 2 8
        9 4 7 3
        3 8 6 5
      INPUT
    end

    it { is_expected.to eq(9) }
  end
end
