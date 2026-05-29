-- name: HEDGEHOG
-- alternates between cooperate and defect, but starts with defect

function decide(round, history)
    if round % 2 == 1 then
        return "defect"
    else
        return "cooperate"
    end
end
