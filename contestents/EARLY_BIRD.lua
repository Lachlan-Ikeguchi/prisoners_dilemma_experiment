-- name: EARLY BIRD
-- cooperates more in early rounds, defects more in late rounds

function decide(round, history)
    if round <= 30 then
        return "cooperate"
    elseif round <= 70 then
        math.randomseed(os.time())
        if math.random() < 0.5 then
            return "cooperate"
        else
            return "defect"
        end
    else
        return "defect"
    end
end
