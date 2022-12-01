# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    module Validators
      class HairColour
        def valid?(value)
          /\A#[0-9a-f]{6}\Z/.match?(value)
        end
      end
    end
  end
end
