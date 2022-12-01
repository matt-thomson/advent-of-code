# frozen_string_literal: true

require 'advent_of_code_2020/day03/forest'

RSpec.describe AdventOfCode2020::Day03::Forest do
  subject(:forest) { described_class.new(fixture_path) }

  let(:fixture_path) { 'spec/fixtures/day03.txt' }

  describe '#tree?' do
    subject(:tree) { forest.tree?(x, y) }

    context 'with a tree in the initial area' do
      let(:x) { 1 }
      let(:y) { 2 }

      it { is_expected.to be(true) }
    end

    context 'with an open space in the initial area' do
      let(:x) { 2 }
      let(:y) { 2 }

      it { is_expected.to be(false) }
    end

    context 'with a tree beyond the initial area' do
      let(:x) { 12 }
      let(:y) { 2 }

      it { is_expected.to be(true) }
    end

    context 'with an open space beyond the initial area' do
      let(:x) { 13 }
      let(:y) { 2 }

      it { is_expected.to be(false) }
    end
  end
end
