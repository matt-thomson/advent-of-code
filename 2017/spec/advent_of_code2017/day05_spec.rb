# frozen_string_literal: true

require 'advent_of_code2017/day05'

RSpec.describe AdventOfCode2017::Day05 do
  describe '.solve_part_one' do
    subject { described_class.solve_part_one(input) }

    let(:input) do
      <<~INPUT
        0
        3
        0
        1
        -3
      INPUT
    end

    it { is_expected.to eq(5) }
  end
end
