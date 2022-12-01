# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/birth_year'

RSpec.describe AdventOfCode2020::Day04::Validators::BirthYear do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { '2002' }

      it { is_expected.to be(true) }
    end

    context 'with a year that is too late' do
      let(:input) { '2003' }

      it { is_expected.to be(false) }
    end

    context 'with a year that is too early' do
      let(:input) { '1919' }

      it { is_expected.to be(false) }
    end

    context 'with non-digits' do
      let(:input) { 'abcd' }

      it { is_expected.to be(false) }
    end
  end
end
