import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.annotation.Keyword as Keyword
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testcase.TestCaseFactory.*
import com.kms.katalon.core.context.TestCaseContext as TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext as TestSuiteContext
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords


//CustomKeywords.'methods.APIKeywords.TerminalPNR'()

CustomKeywords.'methods.APIKeywords.TestExecution'()

def Eligibility = WS.sendRequestAndVerify(findTestObject('Object Repository/OneWayTrip/S_GetEligibilityRequestbyTKT'))

 
CustomKeywords.'methods.APIKeywords.ErrorMessage'(Eligibility)
 

def slurper = new groovy.json.JsonSlurper()

def sEligibility = slurper.parseText(Eligibility.getResponseBodyContent())

def sResponse = sEligibility.findAll()

sResponse.each{ 
	
        def sKeys = " $it.key "

        sKeys = sKeys.toString().trim()

       // println(sKeys)

        KeywordUtil.markPassed('-------Validation for EligibilitybyTicket-----------')

        if (sKeys == 'TicketChangeEligibilityListResponse') {
            def sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].exchangeable

            if ((sExchangable == 'All') || (sExchangable == 'Some')) {
                KeywordUtil.markPassed('Exchangeable Value is available in Response as :- ' + sExchangable)

                String sPTC = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].PassengerTypeCode

                if ((sPTC.length() > 0) && (sPTC != null)) {
                    KeywordUtil.markPassed('PTC Value is available in Response as:- ' + sPTC)
                } else {
                    KeywordUtil.markFailed('Error Response returned or PTC Value is is not available')
                }
                
                String sTicketValue = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Identifier.value

                if ((sTicketValue.length() > 0) && (sTicketValue != null)) {
                    KeywordUtil.markPassed('TicketNumber Value is available in Response as :-' + sTicketValue)
                } else {
                    KeywordUtil.markFailed('Error Response returned or ticketNumber Value is is not available')
                }
                
                String sTraceID = sEligibility.TicketChangeEligibilityListResponse.traceId

                if ((sTraceID.length() > 0) && (sTraceID != null)) {
                    KeywordUtil.markPassed('traceid Value is available in Response as :-' + sTraceID)
                } else {
                    KeywordUtil.markFailed('Error Response returned or traceid Value is is not available')
                }
                
                String sMaxChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Penalties.Change[
                0].Penalty[0].Amount.value

                if ((sMaxChangeFee.length() > 0) && (sMaxChangeFee != null)) {
                    KeywordUtil.markPassed('maximumChangeFee value  is available in Response as :- ' + sMaxChangeFee)
                } else {
                    KeywordUtil.markFailed('Error Response returned or maximumChangeFee value  is is not available')
                }
                
                String sMinChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Penalties.Change[
                1].Penalty[0].Amount.value

                if ((sMinChangeFee.length() > 0) && (sMinChangeFee != null)) {
                    KeywordUtil.markPassed('minimumChangeFee value  is available in Response as :- ' + sMinChangeFee)
                } else {
                    KeywordUtil.markFailed('Error Response returned or minimumChangeFee value  is is not available')
                }
                
                //if (sKeys == 'CancelPermitted') {
                String sCancelPermittedValue = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[
                0].Penalties.Cancel[0].Penalty[0].Amount.value

                if ((sCancelPermittedValue.length() > 0) && (sCancelPermittedValue != null)) {
                    KeywordUtil.markPassed('CancelPermitted value  is available in Response as :- ' + sCancelPermittedValue) //KeywordUtil.logInfo('The PNR is not exchangeable')
                } else {
                    KeywordUtil.markFailed('Error Response returned or CancelPermitted value  is is not available')
                }
            } //   }
        } else {
            KeywordUtil.markFailedAndStop('Error Response returned or Exchangeable Value is is not available')
        }
    }


