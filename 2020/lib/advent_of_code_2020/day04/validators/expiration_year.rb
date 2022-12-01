# frozen_string_literal: true

require_relative './year'

module AdventOfCode2020
  module Day04
    module Validators
      class ExpirationYear < Year
        def initialize
          super(2020, 2030)
        end
      end
    end
  end
end
