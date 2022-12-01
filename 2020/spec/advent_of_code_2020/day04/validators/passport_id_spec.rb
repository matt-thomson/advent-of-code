# frozen_string_literal: true

require 'advent_of_code_2020/day04/validators/passport_id'

RSpec.describe AdventOfCode2020::Day04::Validators::PassportID do
  subject(:validator) { described_class.new }

  describe '#valid?' do
    subject(:valid?) { validator.valid?(input) }

    context 'with a valid input' do
      let(:input) { '000000001' }

      it { is_expected.to be(true) }
    end

    context 'with an invalid character' do
      let(:input) { '00000000a' }

      it { is_expected.to be(false) }
    end

    context 'with the wrong length' do
      let(:input) { '00000001' }

      it { is_expected.to be(false) }
    end
  end
end
