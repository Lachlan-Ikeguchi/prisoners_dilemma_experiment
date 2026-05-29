-- name: KIND
-- cooperates unless opponent defected in the last round

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "defect" then
        return "defect"
    else
        return "cooperate"
    end
end
