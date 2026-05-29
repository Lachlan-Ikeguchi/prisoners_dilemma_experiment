-- name: DRIFTER
-- slowly shifts from cooperation to defection over the game

function decide(round, history)
    local cooperate_prob = 1.0 - (round / 100.0)
    math.randomseed(os.time() + round)
    if math.random() < cooperate_prob then
        return "cooperate"
    else
        return "defect"
    end
end
