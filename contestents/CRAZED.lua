-- name: CRAZED
-- completely unpredictable

function decide(round, history)
    math.randomseed(os.time() * round)
    if math.random() < 0.5001 then
        return "cooperate"
    else
        return "defect"
    end
end
