-- name: ALTERNATOR
-- alternates between cooperate and defect every round

function decide(round, history)
    if round % 2 == 1 then
        return "cooperate"
    else
        return "defect"
    end
end
