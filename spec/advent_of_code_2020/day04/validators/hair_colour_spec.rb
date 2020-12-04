# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/hair_colour'

RSpec.describe AdventOfCode2020::Day04::Validators::HairColour do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { '#123abc' }

      it { is_expected.to eq(true) }
    end

    context 'with an invalid character' do
      let(:input) { '#123abz' }

      it { is_expected.to eq(false) }
    end

    context 'with no leading hash' do
      let(:input) { '123abc' }

      it { is_expected.to eq(false) }
    end

    context 'with the wrong length' do
      let(:input) { '#123ab' }

      it { is_expected.to eq(false) }
    end
  end
end
