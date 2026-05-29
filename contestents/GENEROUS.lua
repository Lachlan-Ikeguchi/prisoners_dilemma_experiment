-- name: GENEROUS
-- mostly cooperates, only retaliates with 33% chance after defection

math.randomseed(os.time())

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        if math.random() < 0.33 then
            return "defect"
        else
            return "cooperate"
        end
    else
        return "cooperate"
    end
end
